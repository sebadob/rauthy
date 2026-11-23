use crate::common::{
    cookie_csrf_headers_from_res_direct, get_auth_headers, get_backend_url, get_solved_pow,
};
use crate::{expect_status, expect_status_json};
use actix_web::{App, HttpResponse, HttpServer, web};
use rauthy_api_types::auth_providers::{
    ClaimMapping, ClaimMappingTyp, ProviderCallbackRequest, ProviderLoginRequest,
};
use rauthy_api_types::users::UpdateUserRequest;
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_encode, new_store_id};
use reqwest::header::{self, HeaderMap, HeaderValue};
use std::error::Error;
use std::net::TcpStream;
use std::sync::{LazyLock, OnceLock};
use std::thread;
use std::time::Duration;

/// Every name this file's tests create carries this suffix, so two runs on one backend never share
/// a role, a group, or a federated user.
static SUFFIX: LazyLock<String> = LazyLock::new(|| new_store_id().to_lowercase());
static ROLE: LazyLock<String> = LazyLock::new(|| format!("cm_test_role_{}", *SUFFIX));
static GROUP: LazyLock<String> = LazyLock::new(|| format!("cm test group {}", *SUFFIX));
/// A role name that contains `rauthy_admin` without being admin-scoped.
const ADMIN_ALIKE: &str = "rauthy_administrator";
/// A group name without a space: the name rules of a role, which the group admin role string must
/// match, do not allow whitespace.
static GROUP_ADMIN_GROUP: LazyLock<String> = LazyLock::new(|| format!("cm_test_group_{}", *SUFFIX));
const ISSUER: &str = "http://localhost:1/claim-mapping-test";

/// Nothing on this port listens: the config tests never reach the upstream at all.
const UNREACHABLE: &str = "http://localhost:1";

/// The upstream endpoints and the onboarding flag differ between the config tests, which never
/// reach an upstream, and the login tests, which must reach the mock IdP below.
struct ProviderSpec<'a> {
    issuer: &'a str,
    base: &'a str,
    auto_onboarding: bool,
}

const CONFIG_SPEC: ProviderSpec<'static> = ProviderSpec {
    issuer: ISSUER,
    base: UNREACHABLE,
    auto_onboarding: false,
};

async fn create_provider(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    mappings: serde_json::Value,
    spec: &ProviderSpec<'_>,
) -> Result<String, Box<dyn Error>> {
    let res = http
        .post(format!("{url}/providers/create"))
        .headers(headers.clone())
        .json(&serde_json::json!({
            "name": "Claim Mapping Test",
            "typ": "custom",
            "enabled": true,
            "issuer": spec.issuer,
            "authorization_endpoint": format!("{}/authorize", spec.base),
            "token_endpoint": format!("{}/token", spec.base),
            "userinfo_endpoint": format!("{}/userinfo", spec.base),
            "use_pkce": true,
            "client_secret_basic": false,
            "client_secret_post": false,
            "auto_onboarding": spec.auto_onboarding,
            "auto_link": false,
            "hide_login": false,
            "client_id": "rauthy",
            "scope": "openid",
            "claim_mappings": mappings,
        }))
        .send()
        .await?;
    let res = expect_status!(res, 200);
    Ok(res.json::<serde_json::Value>().await?["id"]
        .as_str()
        .expect("the provider id")
        .to_string())
}

/// Roles and groups are deleted by their ID, never by their name, and a name that does not exist
/// is a no-op.
async fn delete_by_name(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    endpoint: &str,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    let res = http
        .get(format!("{url}/{endpoint}"))
        .headers(headers.clone())
        .send()
        .await?;
    let all = expect_status_json!(res, 200, Vec<serde_json::Value>);
    for obj in all {
        if obj["name"].as_str() == Some(name) {
            let id = obj["id"].as_str().expect("the id");
            let res = http
                .delete(format!("{url}/{endpoint}/{id}"))
                .headers(headers.clone())
                .send()
                .await?;
            expect_status!(res, 200);
        }
    }

    Ok(())
}

async fn delete_user_by_email(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    let res = http
        .get(format!("{url}/users"))
        .headers(headers.clone())
        .send()
        .await?;
    let users = expect_status_json!(res, 200, Vec<serde_json::Value>);
    for user in users {
        if user["email"].as_str() == Some(email) {
            let id = user["id"].as_str().expect("the user id");
            let res = http
                .delete(format!("{url}/users/{id}"))
                .headers(headers.clone())
                .send()
                .await?;
            // A deleted user answers 204, not 200.
            expect_status!(res, 204);
        }
    }

    Ok(())
}

/// The provider `issuer` is unique, and a name can only exist once: anything a previous, failed run
/// left behind is removed before a test starts.
async fn delete_leftovers(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    issuer: &str,
    role: &str,
    group: &str,
    user_email: &str,
) -> Result<(), Box<dyn Error>> {
    let res = http
        .post(format!("{url}/providers"))
        .headers(headers.clone())
        .send()
        .await?;
    let providers = expect_status_json!(res, 200, Vec<serde_json::Value>);
    for provider in providers {
        if provider["issuer"].as_str() == Some(issuer) {
            let id = provider["id"].as_str().expect("the provider id");
            let res = http
                .delete(format!("{url}/providers/{id}"))
                .headers(headers.clone())
                .send()
                .await?;
            expect_status!(res, 200);
        }
    }

    delete_by_name(http, url, headers, "roles", role).await?;
    delete_by_name(http, url, headers, "groups", group).await?;
    delete_user_by_email(http, url, headers, user_email).await?;

    Ok(())
}

/// `ProviderResponse` is a response type without `Deserialize`, so the assertions work on the
/// JSON of the provider list directly.
async fn get_claim_mappings(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    id: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let res = http
        .post(format!("{url}/providers"))
        .headers(headers.clone())
        .send()
        .await?;
    let providers = expect_status_json!(res, 200, Vec<serde_json::Value>);
    providers
        .into_iter()
        .find(|p| p["id"].as_str() == Some(id))
        .map(|p| p["claim_mappings"].clone())
        .ok_or("provider not found".into())
}

async fn put(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    id: &str,
    mappings: serde_json::Value,
    spec: &ProviderSpec<'_>,
) -> Result<u16, Box<dyn Error>> {
    let res = http
        .put(format!("{url}/providers/{id}"))
        .headers(headers.clone())
        .json(&serde_json::json!({
            "name": "Claim Mapping Test",
            "typ": "custom",
            "enabled": true,
            "issuer": spec.issuer,
            "authorization_endpoint": format!("{}/authorize", spec.base),
            "token_endpoint": format!("{}/token", spec.base),
            "userinfo_endpoint": format!("{}/userinfo", spec.base),
            "use_pkce": true,
            "client_secret_basic": false,
            "client_secret_post": false,
            "auto_onboarding": spec.auto_onboarding,
            "auto_link": false,
            "hide_login": false,
            "client_id": "rauthy",
            "scope": "openid",
            "claim_mappings": mappings,
        }))
        .send()
        .await?;
    Ok(res.status().as_u16())
}

fn rule(typ: ClaimMappingTyp, path: &str, value: &str, target: &str) -> ClaimMapping {
    ClaimMapping {
        claim_path: path.to_string(),
        claim_value: value.to_string(),
        typ,
        target: target.to_string(),
    }
}

#[tokio::test]
async fn test_provider_claim_mappings() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    delete_leftovers(
        &http,
        &backend_url,
        &auth_headers,
        ISSUER,
        ROLE.as_str(),
        GROUP.as_str(),
        "",
    )
    .await?;
    delete_by_name(&http, &backend_url, &auth_headers, "roles", ADMIN_ALIKE).await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_ADMIN_GROUP.as_str(),
    )
    .await?;

    // The targets must exist before the config can be saved.
    for (url, payload) in [
        (
            format!("{backend_url}/roles"),
            serde_json::json!({ "role": ROLE.as_str() }),
        ),
        (
            format!("{backend_url}/groups"),
            serde_json::json!({ "group": GROUP.as_str() }),
        ),
        (
            format!("{backend_url}/roles"),
            serde_json::json!({ "role": ADMIN_ALIKE }),
        ),
        (
            format!("{backend_url}/groups"),
            serde_json::json!({ "group": GROUP_ADMIN_GROUP.as_str() }),
        ),
    ] {
        let res = http
            .post(&url)
            .headers(auth_headers.clone())
            .json(&payload)
            .send()
            .await?;
        expect_status!(res, 200);
    }

    let mappings = serde_json::json!([
        { "claim_path": "$.groups[*]", "claim_value": "^cm:", "typ": "role", "target": ROLE.as_str() },
        { "claim_path": "$..['cm-groups']", "claim_value": "~^cm:admin$", "typ": "group", "target": GROUP.as_str() },
    ]);
    let id = create_provider(&http, &backend_url, &auth_headers, mappings, &CONFIG_SPEC).await?;

    // The rules survive the round-trip through the JSON column unchanged.
    assert_eq!(
        get_claim_mappings(&http, &backend_url, &auth_headers, &id).await?,
        serde_json::to_value(vec![
            rule(ClaimMappingTyp::Role, "$.groups[*]", "^cm:", ROLE.as_str()),
            rule(
                ClaimMappingTyp::Group,
                "$..['cm-groups']",
                "~^cm:admin$",
                GROUP.as_str()
            ),
        ])?,
        "the claim mappings must round-trip"
    );

    // A target that does not exist is rejected and named.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": "role", "target": "cm_missing" }]),
            &CONFIG_SPEC,
        )
        .await?,
        400
    );

    // The full admin role is admin-scoped: it belongs to the provider's own admin claim mapping,
    // so no generic rule may target it.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": "role", "target": "rauthy_admin" }]),
            &CONFIG_SPEC,
        )
        .await?,
        400,
        "`rauthy_admin` is admin-scoped"
    );

    // The group-admin delegates are valid generic targets. `rauthy_admin:<group>` is a role name,
    // and `rauthy_admin:*` is the glob over every group.
    for target in [
        format!("rauthy_admin:{}", GROUP_ADMIN_GROUP.as_str()),
        "rauthy_admin:*".to_string(),
    ] {
        assert_eq!(
            put(
                &http,
                &backend_url,
                &auth_headers,
                &id,
                serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": "role", "target": target }]),
                &CONFIG_SPEC,
            )
            .await?,
            200,
            "target {target} must grant a delegated group admin"
        );
    }

    // It is a role, never a group, and the group behind it must exist. An empty group part names
    // nothing.
    for (typ, target) in [
        (
            "group",
            format!("rauthy_admin:{}", GROUP_ADMIN_GROUP.as_str()),
        ),
        ("role", "rauthy_admin:cm_missing_group".to_string()),
        ("role", "rauthy_admin:".to_string()),
    ] {
        assert_eq!(
            put(
                &http,
                &backend_url,
                &auth_headers,
                &id,
                serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": typ, "target": target }]),
                &CONFIG_SPEC,
            )
            .await?,
            400,
            "target {target} must be rejected as a {typ}"
        );
    }

    // A `,` separates the entries of a user's role / group CSV, so it can never be part of a
    // target, even though the name rules of a role and a group both allow it.
    for (typ, target) in [
        ("role", format!("{},cm_test", ROLE.as_str())),
        ("group", format!("{},cm_test", GROUP.as_str())),
    ] {
        assert_eq!(
            put(
                &http,
                &backend_url,
                &auth_headers,
                &id,
                serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": typ, "target": target }]),
                &CONFIG_SPEC,
            )
            .await?,
            400,
            "target {target} must not contain a comma"
        );
    }

    // A role that only contains the name is an ordinary target.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.roles", "claim_value": "x", "typ": "role", "target": ADMIN_ALIKE }]),
            &CONFIG_SPEC,
        )
        .await?,
        200
    );

    // An invalid regex is rejected at config save, not on a login.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.roles", "claim_value": "~(unclosed", "typ": "role", "target": ROLE.as_str() }]),
            &CONFIG_SPEC,
        )
        .await?,
        400
    );

    // A path outside the supported grammar is rejected: filters are not part of the grammar.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.groups[?@.a=='x']", "claim_value": "x", "typ": "role", "target": ROLE.as_str() }]),
            &CONFIG_SPEC,
        )
        .await?,
        400
    );

    // A `path` and its `value` must be set together.
    let res = http
        .put(format!("{backend_url}/providers/{id}"))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({
            "name": "Claim Mapping Test",
            "typ": "custom",
            "enabled": true,
            "issuer": ISSUER,
            "authorization_endpoint": format!("{UNREACHABLE}/authorize"),
            "token_endpoint": format!("{UNREACHABLE}/token"),
            "userinfo_endpoint": format!("{UNREACHABLE}/userinfo"),
            "use_pkce": true,
            "client_secret_basic": false,
            "client_secret_post": false,
            "auto_onboarding": false,
            "auto_link": false,
            "hide_login": false,
            "client_id": "rauthy",
            "scope": "openid",
            "admin_claim_path": "$.roles[*]",
            "admin_claim_value": null,
        }))
        .send()
        .await?;
    assert_eq!(res.status().as_u16(), 400);

    // More than the supported amount of rules is rejected.
    let too_many = serde_json::Value::Array(
        (0..33)
            .map(|_| {
                serde_json::json!({
                    "claim_path": "$.roles",
                    "claim_value": "x",
                    "typ": "role",
                    "target": ROLE.as_str(),
                })
            })
            .collect(),
    );
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            too_many,
            &CONFIG_SPEC
        )
        .await?,
        400
    );

    // A valid update replaces the whole set, and an empty one clears the column.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.roles[*]", "claim_value": "^^literal", "typ": "group", "target": GROUP.as_str() }]),
            &CONFIG_SPEC,
        )
        .await?,
        200
    );
    assert_eq!(
        get_claim_mappings(&http, &backend_url, &auth_headers, &id).await?,
        serde_json::to_value(vec![rule(
            ClaimMappingTyp::Group,
            "$.roles[*]",
            "^^literal",
            GROUP.as_str()
        )])?
    );

    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([]),
            &CONFIG_SPEC
        )
        .await?,
        200
    );
    assert_eq!(
        get_claim_mappings(&http, &backend_url, &auth_headers, &id).await?,
        serde_json::json!([]),
        "an empty set must clear the column"
    );

    let res = http
        .delete(format!("{backend_url}/providers/{id}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    delete_by_name(&http, &backend_url, &auth_headers, "roles", ROLE.as_str()).await?;
    delete_by_name(&http, &backend_url, &auth_headers, "roles", ADMIN_ALIKE).await?;
    delete_by_name(&http, &backend_url, &auth_headers, "groups", GROUP.as_str()).await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_ADMIN_GROUP.as_str(),
    )
    .await?;

    Ok(())
}

// --- The mapping on the login path -------------------------------------------------------------

const ISSUER_LOGIN: &str = "http://localhost:1/claim-mapping-login";
static ROLE_LOGIN: LazyLock<String> = LazyLock::new(|| format!("cm_login_role_{}", *SUFFIX));
static GROUP_LOGIN: LazyLock<String> = LazyLock::new(|| format!("cm login group {}", *SUFFIX));
/// The delegated admin of `GROUP_LOGIN_ADMIN.as_str()`, granted by a generic rule. The group name has no
/// space, because the group admin target is a role name and the role rules forbid whitespace.
static GROUP_LOGIN_ADMIN: LazyLock<String> =
    LazyLock::new(|| format!("cm_login_admin_{}", *SUFFIX));
static GROUP_ADMIN_ROLE: LazyLock<String> =
    LazyLock::new(|| format!("rauthy_admin:{}", GROUP_LOGIN_ADMIN.as_str()));
static FED_EMAIL: LazyLock<String> =
    LazyLock::new(|| format!("cm-federated-{}@localhost", *SUFFIX));
/// A role that only the local admin grants. It is never named by any rule, so the reconcile must
/// never touch it.
const LOCAL_ROLE: &str = "bs_role_1";
/// The port the mock IdP bound to, set by [`serve_mock_idp`].
static MOCK_PORT: OnceLock<u16> = OnceLock::new();
const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";

/// The mock IdP's base URL, from the port it bound to. Leaked so a [`ProviderSpec`] can hold it
/// as a `&'static str`.
fn mock_base() -> &'static str {
    let port = *MOCK_PORT.get().expect("the mock IdP to be running");
    Box::leak(format!("http://127.0.0.1:{port}").into_boxed_str())
}

fn login_spec() -> ProviderSpec<'static> {
    ProviderSpec {
        issuer: ISSUER_LOGIN,
        base: mock_base(),
        auto_onboarding: true,
    }
}

/// The upstream IdP for the login tests. Only the backend's own `/token` request ever reaches it,
/// and it answers with the claim set the request's `code` names: `claims-granted` has both values
/// the rules below map, `claims-revoked` has neither.
fn serve_mock_idp() {
    thread::spawn(|| {
        actix_web::rt::System::new().block_on(async {
            // Port 0: the OS picks a free port, so the mock never collides with anything else.
            let server = HttpServer::new(|| App::new().route("/token", web::post().to(mock_token)))
                .bind(("127.0.0.1", 0))
                .expect("an ephemeral port for the mock IdP");
            let port = server
                .addrs()
                .first()
                .expect("the bound mock IdP address")
                .port();
            MOCK_PORT
                .set(port)
                .expect("the mock port to be set only once");
            server.run().await.expect("the mock IdP to start");
        });
    });

    for _ in 0..50 {
        if let Some(port) = MOCK_PORT.get()
            && TcpStream::connect(("127.0.0.1", *port)).is_ok()
        {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
    panic!("the mock IdP did not start");
}

fn upstream_claims(code: &str) -> serde_json::Value {
    let groups = if code == "claims-granted" {
        serde_json::json!(["engineering", "on-call"])
    } else {
        serde_json::json!(["on-call"])
    };
    let tgchat = if code == "claims-granted" {
        serde_json::json!(["tgchat:admin"])
    } else {
        serde_json::json!(["other:admin"])
    };
    serde_json::json!({
        "iss": ISSUER_LOGIN,
        "sub": "cm-fed-1",
        "email": FED_EMAIL.as_str(),
        "email_verified": true,
        "given_name": "Cm",
        "family_name": "Federated",
        "locale": "en",
        "groups": groups,
        "tgchat": tgchat,
    })
}

async fn mock_token(body: web::Bytes) -> HttpResponse {
    let body = String::from_utf8_lossy(&body);
    let code = body
        .split('&')
        .find_map(|kv| kv.strip_prefix("code="))
        .unwrap_or_default();
    // The backend only decodes the payload of this token, it is never signature-validated.
    let id_token = format!(
        "e30.{}.e30",
        base64_url_no_pad_encode(upstream_claims(code).to_string().as_bytes())
    );
    HttpResponse::Ok().json(serde_json::json!({
        "access_token": null,
        "id_token": id_token,
        "error": null,
        "error_description": null,
    }))
}

/// One full federated login: an init session, `/providers/login`, then the upstream callback that
/// carries `code`, which names the claim set the mock IdP answers with.
async fn federated_login(
    http: &reqwest::Client,
    url: &str,
    provider_id: &str,
    code: &str,
) -> Result<(), Box<dyn Error>> {
    let res = http.post(format!("{url}/oidc/session")).send().await?;
    let mut headers = cookie_csrf_headers_from_res_direct(res).await?;

    let res = http
        .post(format!("{url}/providers/login"))
        .headers(headers.clone())
        .json(&ProviderLoginRequest {
            email: None,
            client_id: "rauthy".to_string(),
            redirect_uri: format!("{url}/oidc/callback"),
            scopes: None,
            state: None,
            nonce: None,
            // The `rauthy` client requires PKCE, so the downstream challenge must be carried.
            code_challenge: Some(base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes()))),
            code_challenge_method: Some("S256".to_string()),
            resource: None,
            pow: get_solved_pow().await,
            provider_id: provider_id.to_string(),
            pkce_challenge: base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes())),
            handle: None,
            fwda: None,
        })
        .send()
        .await?;
    let res = expect_status!(res, 202);

    let callback_cookie = res
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|c| c.to_str().ok()?.split_once(';').map(|(c, _)| c.to_string()))
        .find(|c| c.contains("UpstreamAuthCallback="))
        .expect("the upstream callback cookie");
    let location = reqwest::Url::parse(
        res.headers()
            .get(header::LOCATION)
            .expect("Location header")
            .to_str()?,
    )?;
    let callback_id = location
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.into_owned())
        .expect("state in the upstream Location");
    let xsrf_token = res.text().await?;

    let cookies = format!(
        "{}; {callback_cookie}",
        headers
            .get(header::COOKIE)
            .expect("the session cookie")
            .to_str()?
    );
    headers.insert(header::COOKIE, HeaderValue::from_str(&cookies)?);

    let res = http
        .post(format!("{url}/providers/callback"))
        .headers(headers)
        .json(&ProviderCallbackRequest {
            state: callback_id,
            code: code.to_string(),
            xsrf_token,
            pkce_verifier: PKCE_VERIFIER.to_string(),
            iss_atproto: None,
        })
        .send()
        .await?;
    // A completed login is 202 with the auth code, or 205 when the new account needs a password.
    let status = res.status().as_u16();
    assert!(
        status == 202 || status == 205,
        "the federated login must complete - got {status}: {:?}",
        res.text().await?
    );

    Ok(())
}

async fn get_federated_user(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let res = http
        .get(format!("{url}/users"))
        .headers(headers.clone())
        .send()
        .await?;
    let users = expect_status_json!(res, 200, Vec<serde_json::Value>);
    let user = users
        .into_iter()
        .find(|u| u["email"].as_str() == Some(FED_EMAIL.as_str()))
        .ok_or("the federated user must exist")?;
    let id = user["id"].as_str().expect("the user id").to_string();

    let res = http
        .get(format!("{url}/users/{id}"))
        .headers(headers.clone())
        .send()
        .await?;
    Ok(expect_status_json!(res, 200, serde_json::Value))
}

fn names(user: &serde_json::Value, field: &str) -> Vec<String> {
    user[field]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// Grants `LOCAL_ROLE` next to the mapped role, exactly the way a local admin would.
async fn grant_local_role(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
) -> Result<(), Box<dyn Error>> {
    let user = get_federated_user(http, url, headers).await?;
    let id = user["id"].as_str().expect("the user id").to_string();

    let res = http
        .put(format!("{url}/users/{id}"))
        .headers(headers.clone())
        .json(&UpdateUserRequest {
            email: FED_EMAIL.clone(),
            given_name: user["given_name"].as_str().map(String::from),
            family_name: user["family_name"].as_str().map(String::from),
            language: None,
            password: None,
            roles: vec![LOCAL_ROLE.to_string(), ROLE_LOGIN.clone()],
            groups: Some(vec![GROUP_LOGIN.clone()]),
            enabled: true,
            email_verified: true,
            user_expires: None,
            user_values: None,
        })
        .send()
        .await?;
    expect_status!(res, 200, "the local role grant must succeed");

    Ok(())
}

/// The login path itself: rules are compiled once per provider, applied on every login, and the
/// reconcile only ever removes the names the provider's own rules manage.
#[tokio::test]
async fn test_provider_claim_mappings_federated_login() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    delete_leftovers(
        &http,
        &backend_url,
        &auth_headers,
        ISSUER_LOGIN,
        ROLE_LOGIN.as_str(),
        GROUP_LOGIN.as_str(),
        FED_EMAIL.as_str(),
    )
    .await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_LOGIN_ADMIN.as_str(),
    )
    .await?;
    serve_mock_idp();

    for (url, payload) in [
        (
            format!("{backend_url}/roles"),
            serde_json::json!({ "role": ROLE_LOGIN.as_str() }),
        ),
        (
            format!("{backend_url}/groups"),
            serde_json::json!({ "group": GROUP_LOGIN.as_str() }),
        ),
        (
            format!("{backend_url}/groups"),
            serde_json::json!({ "group": GROUP_LOGIN_ADMIN.as_str() }),
        ),
    ] {
        let res = http
            .post(&url)
            .headers(auth_headers.clone())
            .json(&payload)
            .send()
            .await?;
        expect_status!(res, 200);
    }

    let mappings = serde_json::json!([
        { "claim_path": "$.groups[*]", "claim_value": "engineering", "typ": "role", "target": ROLE_LOGIN.as_str() },
        { "claim_path": "$.tgchat[*]", "claim_value": "^tgchat:", "typ": "group", "target": GROUP_LOGIN.as_str() },
        { "claim_path": "$.tgchat[*]", "claim_value": "^tgchat:", "typ": "role", "target": GROUP_ADMIN_ROLE.as_str() },
    ]);
    let id = create_provider(&http, &backend_url, &auth_headers, mappings, &login_spec()).await?;

    // The first login creates the federated account with exactly the matched names.
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![ROLE_LOGIN.clone(), GROUP_ADMIN_ROLE.clone()],
        "the mapped role and the mapped group admin must be granted"
    );
    assert_eq!(
        names(&user, "groups"),
        vec![GROUP_LOGIN.clone()],
        "the mapped group must be granted"
    );

    // A name this provider maps belongs to the provider. Upstream revokes both mapped values, and
    // a login without a match takes back the locally granted role that a rule names, while the
    // locally granted role no rule names stays.
    grant_local_role(&http, &backend_url, &auth_headers).await?;
    federated_login(&http, &backend_url, &id, "claims-revoked").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string()],
        "a locally granted name a rule maps is provider-owned, a name no rule maps is not touched"
    );
    assert!(
        names(&user, "groups").is_empty(),
        "the provider-managed group must be removed - {:?}",
        user["groups"]
    );

    // Saving new rules recompiles: the next login uses the new set, and a target no rule names
    // is not granted again.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([
                { "claim_path": "$.groups[*]", "claim_value": "engineering", "typ": "role", "target": ROLE_LOGIN.as_str() }
            ]),
            &login_spec(),
        )
        .await?,
        200
    );
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string(), ROLE_LOGIN.clone()],
        "the mapped role is granted again after the re-save"
    );
    assert!(
        names(&user, "groups").is_empty(),
        "a target no rule names must not be granted - {:?}",
        user["groups"]
    );

    // A provider without any rule changes membership on the login path by nothing.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([]),
            &login_spec()
        )
        .await?,
        200
    );
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string(), ROLE_LOGIN.clone()],
        "no rules means no changes to membership"
    );

    // A target deleted after the config save is dropped on the login path, never auto-created:
    // the rule matches, the role behind it is gone.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.groups[*]", "claim_value": "engineering", "typ": "role", "target": ROLE_LOGIN.as_str() }]),
            &login_spec(),
        )
        .await?,
        200
    );
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "roles",
        ROLE_LOGIN.as_str(),
    )
    .await?;
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string()],
        "a matched target that no longer exists is not granted - {:?}",
        user["roles"]
    );

    // A group admin target is a role name that is never a row in the roles table: its group is
    // what has to exist.
    assert_eq!(
        put(
            &http,
            &backend_url,
            &auth_headers,
            &id,
            serde_json::json!([{ "claim_path": "$.tgchat[*]", "claim_value": "^tgchat:", "typ": "role", "target": GROUP_ADMIN_ROLE.as_str() }]),
            &login_spec(),
        )
        .await?,
        200
    );
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string(), GROUP_ADMIN_ROLE.clone()],
        "the mapped group admin must be granted - {:?}",
        user["roles"]
    );

    federated_login(&http, &backend_url, &id, "claims-revoked").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string()],
        "a rule revokes its group admin like any other target - {:?}",
        user["roles"]
    );

    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_LOGIN_ADMIN.as_str(),
    )
    .await?;
    federated_login(&http, &backend_url, &id, "claims-granted").await?;
    let user = get_federated_user(&http, &backend_url, &auth_headers).await?;
    assert_eq!(
        names(&user, "roles"),
        vec![LOCAL_ROLE.to_string()],
        "a group admin without its group is not granted - {:?}",
        user["roles"]
    );

    let res = http
        .delete(format!("{backend_url}/providers/{id}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    delete_user_by_email(&http, &backend_url, &auth_headers, FED_EMAIL.as_str()).await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "roles",
        ROLE_LOGIN.as_str(),
    )
    .await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_LOGIN.as_str(),
    )
    .await?;
    delete_by_name(
        &http,
        &backend_url,
        &auth_headers,
        "groups",
        GROUP_LOGIN_ADMIN.as_str(),
    )
    .await?;

    Ok(())
}
