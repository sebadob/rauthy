use crate::common::{
    check_status, code_state_from_headers, cookie_csrf_headers_from_res, get_auth_headers,
    get_backend_url, CLIENT_ID, CLIENT_SECRET, PASSWORD, USERNAME,
};
use actix_web::{http, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use ed25519_compact::Noise;
use josekit::jwk;
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::UpdateClientRequest;
use rauthy_api_types::oidc::{
    JwkKeyPairAlg, LoginRequest, TokenInfo, TokenRequest, TokenValidationRequest,
};
use rauthy_common::constants::{
    APPLICATION_JSON, DPOP_TOKEN_ENDPOINT, HEADER_DPOP_NONCE, TOKEN_DPOP,
};
use rauthy_common::utils::{
    base64_encode, base64_url_encode, base64_url_no_pad_decode, base64_url_no_pad_encode, get_rand,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::entity::dpop_proof::{DPoPClaims, DPoPHeader};
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPairType, JWKS};
use rauthy_models::JwtTokenType;
use rauthy_service::token_set::TokenSet;
use reqwest::header::AUTHORIZATION;
use ring::digest;
use std::error::Error;
use std::fmt::Write;
use std::ops::Sub;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;

mod common;

// This is a very long running test - run it manually as a single test
// TODO maybe moving it into its own module would work, so it does not block the others
#[tokio::test]
#[ignore]
async fn test_certs() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    eprintln!("backend_url: {}", backend_url);
    let auth_headers = get_auth_headers().await?;

    // get current certs
    let url = format!("{}/oidc/certs", backend_url);
    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);
    let certs = res.json::<JWKS>().await?;
    assert_eq!(certs.keys.len(), 4);

    // for _ in 1..1000 {
    //     aw!(reqwest::get(&url)).unwrap();
    // }
    // rotate JWKs
    let url_rotate = format!("{}/oidc/rotate_jwk", backend_url);
    let res = reqwest::Client::new()
        .post(&url_rotate)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // get all certs and check for the new ones
    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);
    let new_certs = res.json::<JWKS>().await?;
    assert_eq!(new_certs.keys.len(), 8);

    Ok(())
}

#[tokio::test]
async fn test_authorization_code_flow() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();

    // ############################################################################
    // ########################## flow with plain pkce ############################
    // ############################################################################
    println!("\nStarting authorization code flow with plain pkce");

    // Step 1: GET /authorize for the CSRF token and simulate UI login
    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let redirect_uri = "http://localhost:3000/oidc/callback";
    let query = format!(
        "client_id=init_client&redirect_uri={}&response_type=code",
        redirect_uri
    );
    let query_pkce = format!("{}&code_challenge={}", query, challenge_plain);
    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query_pkce);
    let mut res = reqwest::get(&url_auth).await?;
    res = check_status(res, 200).await?;
    let headers = cookie_csrf_headers_from_res(res).await?;

    // Step 2: POST /authorize with CSRF token cookie
    let nonce = get_rand(32);
    let mut req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some("IAmSoWrong1337".to_string()),
        client_id: CLIENT_ID.to_string(),
        redirect_uri: redirect_uri.to_owned(),
        scopes: None,
        state: None,
        nonce: Some(nonce.to_owned()),
        code_challenge: Some(challenge_plain.to_owned()),
        code_challenge_method: Some("plain".to_string()),
    };
    let res = reqwest::Client::new()
        .post(&url_auth)
        .headers(headers.clone())
        .json(&req_login)
        .send()
        .await?;
    // should be 401 - wrong password
    check_status(res, 401).await?;

    req_login.password = Some(PASSWORD.to_string());
    let mut res = reqwest::Client::new()
        .post(&url_auth)
        .headers(headers.clone())
        .json(&req_login)
        .send()
        .await?;
    res = check_status(res, 202).await?;

    // Step 3: extract values from callback location header
    let (code, _) = code_state_from_headers(res)?;
    println!("Extracted code: {:?}", code);

    // Step 4: POST /token with extracted values + CSRF cookie
    let mut req_token = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: Some(code.to_string()),
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
    };
    let url_token = format!("{}/oidc/token", backend_url);
    let res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    // should be 400 - no code verifier given
    check_status(res, 400).await?;

    req_token.code_verifier = Some("IAmSoWrong1337".to_string());
    let res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    // should be 401 - wrong code verifier given
    check_status(res, 401).await?;

    req_token.code_verifier = Some(challenge_plain.to_string());
    let mut res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    res = check_status(res, 200).await?;
    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(ts.id_token.is_some());
    assert!(ts.refresh_token.is_some());
    assert_eq!(ts.expires_in, 60);

    // verify 'nonce' existing in id token
    let id_token = ts.id_token.unwrap();
    println!("id_token: {}", id_token);
    // decode header to get the kid
    let header = josekit::jwt::decode_header(id_token.clone())?;
    let kid = header
        .claim("kid")
        .map(|v| v.to_string().replace('\"', ""))
        .unwrap();

    // retrieve jwk for kid
    let kid_url = format!("{}/oidc/certs/{}", backend_url, kid);
    let res = reqwest::get(&kid_url).await?;
    assert_eq!(res.status(), 200);
    let jwk = res.json::<jwk::Jwk>().await?;
    println!("{:?}", jwk);

    // check signature and get the payload
    let verifier = josekit::jws::RS512.verifier_from_jwk(&jwk).unwrap();
    let (payload, _) = josekit::jwt::decode_with_verifier(&id_token, &verifier)?;

    // finally, check the 'nonce'
    let nonce_claim = payload
        .claim("nonce")
        .map(|v| v.to_string().replace('\"', ""))
        .expect("'nonce' is not set in id token");
    assert_eq!(nonce_claim, nonce);

    // ############################################################################
    // ########################## flow with S256 pkce #############################
    // ############################################################################
    println!("\nStarting authorization code flow with S256 pkce");

    let mut res = reqwest::get(&url_auth).await?;
    res = check_status(res, 200).await?;
    let headers = cookie_csrf_headers_from_res(res).await?;

    let hash = digest::digest(&digest::SHA256, challenge_plain.as_bytes());
    let challenge_s256 = base64_url_encode(hash.as_ref());
    req_login.code_challenge_method = Some("S256".to_string());
    req_login.code_challenge = Some(challenge_s256);
    let mut res = reqwest::Client::new()
        .post(&url_auth)
        .headers(headers)
        .json(&req_login)
        .send()
        .await?;
    res = check_status(res, 202).await?;
    let (code, state) = code_state_from_headers(res)?;
    assert!(state.is_none());

    let res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    // should be 401 - trying to use already used authorization code
    check_status(res, 401).await?;

    req_token.code = Some(code);
    let mut res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    res = check_status(res, 200).await?;

    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(ts.id_token.is_some());
    assert!(ts.refresh_token.is_some());
    assert_eq!(ts.expires_in, 60);

    // ############################################################################
    // ########################## flow without pkce ###############################
    // ############################################################################
    println!("\nStarting authorization code flow without pkce");

    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query);
    let res = reqwest::get(&url_auth).await?;
    // should be 400 - code_challenge is missing
    check_status(res, 400).await?;

    // disable pkce for the init client
    let mut update_client = UpdateClientRequest {
        id: CLIENT_ID.to_string(),
        name: Some("Init Client".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost:3000/oidc/callback".to_string()],
        post_logout_redirect_uris: Some(vec!["http://localhost:8080".to_string()]),
        allowed_origins: Some(vec!["http://localhost:8080".to_string()]),
        enabled: true,
        flows_enabled: vec![
            "authorization_code".to_string(),
            "password".to_string(),
            "client_credentials".to_string(),
            "refresh_token".to_string(),
        ],
        access_token_alg: JwkKeyPairAlg::RS384,
        id_token_alg: JwkKeyPairAlg::EdDSA,
        auth_code_lifetime: 120,
        access_token_lifetime: 60,
        scopes: vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
            "groups".to_string(),
        ],
        default_scopes: vec!["openid".to_string(), "email".to_string()],
        challenges: None,
        force_mfa: false,
        client_uri: None,
        contacts: None,
    };
    let url_client = format!("{}/clients/{}", backend_url, CLIENT_ID);
    let auth_headers = get_auth_headers().await?;
    println!("{:?}", auth_headers);
    let res = reqwest::Client::new()
        .put(&url_client)
        .headers(auth_headers.clone())
        .json(&update_client)
        .send()
        .await?;
    check_status(res, 200).await?;

    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query);
    let mut res = reqwest::get(&url_auth).await?;
    res = check_status(res, 200).await?;
    let headers = cookie_csrf_headers_from_res(res).await?;

    let state = "SomeStateThatShouldBeReturned";
    req_login.code_challenge_method = None;
    req_login.code_challenge = None;
    req_login.state = Some(state.to_string());
    let mut res = reqwest::Client::new()
        .post(&url_auth)
        .headers(headers)
        .json(&req_login)
        .send()
        .await?;
    res = check_status(res, 202).await?;
    let (code, state_check) = code_state_from_headers(res)?;
    println!("1");
    assert_eq!(state, state_check.unwrap());

    req_token.code = Some(code);
    let mut res = reqwest::Client::new()
        .post(&url_token)
        .form(&req_token)
        .send()
        .await?;
    res = check_status(res, 200).await?;

    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(ts.id_token.is_some());
    assert!(ts.refresh_token.is_some());
    assert_eq!(ts.expires_in, 60);

    // now clean up and change back the pkce for the client
    update_client.challenges = Some(vec!["S256".to_string(), "plain".to_string()]);
    let res = reqwest::Client::new()
        .put(&url_client)
        .headers(auth_headers.clone())
        .json(&update_client)
        .send()
        .await?;
    check_status(res, 200).await?;

    Ok(())
}

#[tokio::test]
async fn test_client_credentials_flow() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();

    let mut body = TokenRequest {
        grant_type: "client_credentials".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: None,
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
    };
    let url = format!("{}/oidc/token", backend_url);
    let client = reqwest::Client::new();
    let res = client.post(&url).form(&body).send().await?;
    // should be 401 because of missing client secret
    check_status(res, 400).await?;

    body.client_secret = Some(CLIENT_SECRET.to_string());
    let mut res = client.post(&url).form(&body).send().await?;
    res = check_status(res, 200).await?;

    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert_eq!(ts.token_type, JwtTokenType::Bearer);
    assert_eq!(ts.expires_in, 60);
    // important: no id token for client_credentials and not refresh token
    assert!(ts.id_token.is_none());
    assert!(ts.refresh_token.is_none());

    let payload = TokenValidationRequest {
        token: ts.access_token.clone(),
    };
    validate_token(&ts.access_token, payload).await?;

    Ok(())
}

// This test is a bit messy currently with some code reception and so one - WIP
#[tokio::test]
#[ignore]
async fn test_concurrent_logins() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();

    let fail_count = 10;
    let success_count = 10;

    // ############################################################################
    // ########################## flow with plain pkce ############################
    // ############################################################################
    println!("Starting authorization code flow with plain pkce");

    // Step 1: GET /authorize for the CSRF token and simulate UI login
    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let redirect_uri = "http://localhost:3000/oidc/callback";
    let query = format!(
        "client_id=init_client&redirect_uri={}&response_type=code",
        redirect_uri
    );
    let query_pkce = format!("{}&code_challenge={}", query, challenge_plain);
    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query_pkce);
    let mut res = reqwest::get(&url_auth).await?;
    res = check_status(res, 200).await?;
    let headers = cookie_csrf_headers_from_res(res).await?;

    // Step 2: POST /authorize with CSRF token cookie
    let mut req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some("IAmSoWrong1337".to_string()),
        client_id: CLIENT_ID.to_string(),
        redirect_uri: redirect_uri.to_owned(),
        scopes: None,
        state: None,
        nonce: None,
        code_challenge: Some(challenge_plain.to_owned()),
        code_challenge_method: None,
    };

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..fail_count {
        let url = url_auth.clone();
        let h = headers.clone();
        let req = req_login.clone();
        let handle = std::thread::spawn(move || {
            aw!(reqwest::Client::new()
                .post(&url)
                .headers(h)
                .json(&req)
                .send())
            .unwrap();
        });
        handles.push(handle);
        // std::thread::sleep(Duration::from_millis(100));
    }

    req_login.password = Some(PASSWORD.to_string());
    for _ in 0..success_count {
        let url = url_auth.clone();
        let h = headers.clone();
        let req = req_login.clone();
        let handle = std::thread::spawn(move || {
            aw!(reqwest::Client::new()
                .post(&url)
                .headers(h)
                .json(&req)
                .send())
            .unwrap();
        });
        handles.push(handle);
        // std::thread::sleep(Duration::from_millis(100));
    }

    assert_eq!(handles.len(), fail_count + success_count);
    for h in handles {
        h.join().unwrap();
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let total = end.sub(start).as_millis();
    println!("Total time taken: {}", total);

    assert_eq!(1, 2);

    Ok(())
}

#[tokio::test]
async fn test_password_flow() -> Result<(), Box<dyn Error>> {
    let before = Utc::now().timestamp();

    let url = format!("{}/oidc/token", get_backend_url());
    let mut body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: None,
        code_verifier: None,
        device_code: None,
        username: Some(USERNAME.to_string()),
        password: None,
        refresh_token: None,
    };
    let client = reqwest::Client::new();
    let res = client.post(&url).form(&body).send().await?;
    // should be 400 - no password
    check_status(res, 400).await?;

    body.password = Some("IAmSoWrong1337".to_string());
    let res = client.post(&url).form(&body).send().await?;
    // should be 400 - missing client secret
    check_status(res, 400).await?;

    body.client_secret = Some("NoNoNoSecret1337".to_string());
    let res = client.post(&url).form(&body).send().await?;
    // should be 400 - wrong password
    check_status(res, 401).await?;

    body.password = Some(PASSWORD.to_string());
    let res = client.post(&url).form(&body).send().await?;
    // should be 401 - wrong client secret
    check_status(res, 401).await?;

    body.client_secret = Some(CLIENT_SECRET.to_string());
    let mut res = client.post(&url).form(&body).send().await?;
    // all should be good now
    res = check_status(res, 200).await?;

    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(ts.id_token.is_some());
    assert!(ts.id_token.as_ref().unwrap().len() > 0);
    assert!(ts.refresh_token.is_some());
    assert!(ts.refresh_token.as_ref().unwrap().len() > 0);
    // test token is valid for only 60 seconds to make the refresh token valid immediately
    assert_eq!(ts.expires_in, 60);

    // validate against the backend
    let payload = TokenValidationRequest {
        token: ts.access_token.to_owned(),
    };
    validate_token(&ts.access_token, payload).await?;

    // make sure `auth_time` is handled properly
    let auth_time_refresh = auth_time_from_token(ts.refresh_token.as_ref().unwrap());
    assert!(before <= auth_time_refresh);
    assert!(auth_time_refresh <= Utc::now().timestamp());

    let auth_time_orig = auth_time_from_token(ts.id_token.as_ref().unwrap());
    assert_eq!(auth_time_refresh, auth_time_orig);

    // refresh it
    time::sleep(Duration::from_secs(2)).await;
    let req = TokenRequest {
        grant_type: "refresh_token".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: Some(ts.refresh_token.clone().unwrap()),
    };
    let url = format!("{}/oidc/token", get_backend_url());
    let res = reqwest::Client::new().post(&url).form(&req).send().await?;
    assert_eq!(res.status(), 200);
    let new_ts = res.json::<TokenSet>().await?;
    assert!(new_ts.access_token.len() > 0);
    assert!(new_ts.id_token.is_some());
    assert!(new_ts.id_token.as_ref().unwrap().len() > 0);
    assert!(new_ts.refresh_token.is_some());
    assert!(new_ts.refresh_token.as_ref().unwrap().len() > 0);
    assert_eq!(new_ts.expires_in, 60);

    assert_ne!(ts.refresh_token, new_ts.refresh_token);
    assert_ne!(ts.access_token, new_ts.access_token);
    assert_ne!(ts.id_token, new_ts.id_token);

    let auth_time = auth_time_from_token(new_ts.id_token.as_ref().unwrap());
    assert_eq!(auth_time_orig, auth_time);

    let auth_time_refresh_new = auth_time_from_token(ts.refresh_token.as_ref().unwrap());
    // the `auth_time` for the refresh token must always stay the original one
    assert_eq!(auth_time_orig, auth_time_refresh_new);

    Ok(())
}

#[tokio::test]
async fn test_dpop() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let url = format!("{}/oidc/token", get_backend_url());

    // token request itself
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        refresh_token: None,
    };

    // dpop header
    let kp = ed25519_compact::KeyPair::generate();

    let header = DPoPHeader {
        typ: "dpop+jwt".to_string(),
        alg: rauthy_models::entity::jwk::JwkKeyPairAlg::EdDSA,
        jwk: JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            // DPoP request will not have the 'alg' here but one level higher
            alg: None,
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: Some(base64_url_encode(kp.pk.as_slice())),
        },
        kid: None,
    };
    let mut claims = DPoPClaims {
        jti: "-BwC3ESc6acc2lTc".to_string(),
        htm: http::Method::POST.to_string(),
        htu: DPOP_TOKEN_ENDPOINT.clone().to_string(),
        iat: Utc::now().timestamp(),
        // ath: None,
        nonce: None,
    };

    let fingerprint = header.jwk.fingerprint().unwrap();

    let header_json = serde_json::to_string(&header).unwrap();
    let header_b64 = base64_url_no_pad_encode(header_json.as_bytes());
    let claims_json = serde_json::to_string(&claims).unwrap();
    let claims_b64 = base64_url_no_pad_encode(claims_json.as_bytes());
    let mut dpop_token = format!("{}.{}", header_b64, claims_b64);

    let sig = kp.sk.sign(&dpop_token, Some(Noise::generate()));
    let sig_b64 = base64_url_no_pad_encode(sig.as_ref());
    write!(dpop_token, ".{}", sig_b64).unwrap();
    println!("test signed token without nonce:\n{}", dpop_token);

    let res = client
        .post(&url)
        .header(TOKEN_DPOP, &dpop_token)
        .form(&body)
        .send()
        .await?;
    // this should fail, because nonce is enforced, which we did not provide
    assert_eq!(res.status(), 400);
    let nonce = res
        .headers()
        .get(HEADER_DPOP_NONCE)
        .unwrap()
        .to_str()
        .unwrap();
    println!("nonce we should use: {}", nonce);

    // insert the nonce and rebuild
    claims.nonce = Some(nonce.to_string());

    let claims_json = serde_json::to_string(&claims).unwrap();
    let claims_b64 = base64_url_no_pad_encode(claims_json.as_bytes());
    let mut dpop_token = format!("{}.{}", header_b64, claims_b64);

    let sig = kp.sk.sign(&dpop_token, Some(Noise::generate()));
    let sig_b64 = base64_url_no_pad_encode(sig.as_ref());
    write!(dpop_token, ".{}", sig_b64).unwrap();
    println!("test signed token with nonce:\n{}", dpop_token);

    let mut res = client
        .post(&url)
        .header(TOKEN_DPOP, &dpop_token)
        .form(&body)
        .send()
        .await?;
    // not it should be good
    res = check_status(res, 200).await?;
    // make sure nonce header is still there
    let nonce_new = res
        .headers()
        .get(HEADER_DPOP_NONCE)
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(nonce, nonce_new);

    let ts = res.json::<TokenSet>().await?;
    assert_eq!(ts.token_type, JwtTokenType::DPoP);
    let payload = TokenValidationRequest {
        token: ts.access_token.to_owned(),
    };
    let token_info = validate_token(&ts.access_token, payload).await?;
    assert!(token_info.cnf.is_some());
    assert_eq!(token_info.cnf.unwrap().jkt, fingerprint);

    // refresh it
    time::sleep(Duration::from_secs(1)).await;
    let req = TokenRequest {
        grant_type: "refresh_token".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: Some(ts.refresh_token.clone().unwrap()),
    };

    // without DPoP header, it should fail
    let res = reqwest::Client::new().post(&url).form(&req).send().await?;
    assert_eq!(res.status(), 403);
    let err = res.json::<ErrorResponse>().await.unwrap();
    println!("{:?}", err);
    assert_eq!(err.error, ErrorResponseType::Forbidden);
    assert!(err.message.to_lowercase().contains("dpop"));

    // now a proper refresh with DPoP header
    let res = reqwest::Client::new()
        .post(&url)
        .header(TOKEN_DPOP, dpop_token)
        .form(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let ts = res.json::<TokenSet>().await?;
    assert_eq!(ts.token_type, JwtTokenType::DPoP);
    let payload = TokenValidationRequest {
        token: ts.access_token.to_owned(),
    };
    let token_info = validate_token(&ts.access_token, payload).await?;
    assert!(token_info.cnf.is_some());
    assert_eq!(token_info.cnf.unwrap().jkt, fingerprint);

    Ok(())
}

#[tokio::test]
async fn test_authorization_code_flow_ephemeral_client() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    // host our own ephemeral client
    let handle = serve_ephemeral_client();
    // make sure the http server starts and keeps running
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!handle.is_finished());
    // our ephemeral client's id
    let client_id = "http://127.0.0.1:10080/client";

    // Step 1: GET /authorize for the CSRF token and simulate UI login
    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let hash = digest::digest(&digest::SHA256, challenge_plain.as_bytes());
    let challenge_s256 = base64_url_encode(hash.as_ref());

    let redirect_uri = "http://localhost:3000/oidc/callback";
    let query = format!(
        "client_id={}&redirect_uri={}&response_type=code&scope=openid+profile+email+webid",
        client_id, redirect_uri
    );
    let query_pkce = format!(
        "{}&code_challenge_method=S256&code_challenge={}",
        query, challenge_s256
    );
    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query_pkce);

    // make s simple GET to extract the CSRF token
    let res = reqwest::get(&url_auth).await?;
    assert!(res.status().is_success());
    let headers = cookie_csrf_headers_from_res(res).await?;

    // login and get an authorization code
    let nonce = get_rand(32);
    let req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some(PASSWORD.to_string()),
        client_id: client_id.to_string(),
        redirect_uri: redirect_uri.to_owned(),
        scopes: None,
        state: None,
        nonce: Some(nonce.to_owned()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
    };
    let res = client
        .post(&url_auth)
        .headers(headers.clone())
        .json(&req_login)
        .send()
        .await?;
    assert!(res.status().is_success());
    let (code, state) = code_state_from_headers(res)?;
    assert!(state.is_none());
    println!("Extracted code: {:?}", code);

    // get a token with the code
    let req_token = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: Some(code.to_string()),
        redirect_uri: None,
        client_id: Some(client_id.to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
    };

    let url_token = format!("{}/oidc/token", backend_url);
    let res = client.post(&url_token).form(&req_token).send().await?;
    assert!(res.status().is_success());

    let ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(ts.id_token.is_some());
    assert!(ts.refresh_token.is_some());
    assert_eq!(ts.expires_in, 60);

    // now try to refresh the token
    time::sleep(Duration::from_secs(1)).await;
    let req = TokenRequest {
        grant_type: "refresh_token".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(client_id.to_string()),
        client_secret: None,
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: Some(ts.refresh_token.clone().unwrap()),
    };
    let res = client.post(&url_token).form(&req).send().await?;
    assert!(res.status().is_success());

    let new_ts = res.json::<TokenSet>().await?;
    assert!(ts.access_token.len() > 0);
    assert!(new_ts.id_token.is_some());
    assert!(new_ts.refresh_token.is_some());

    Ok(())
}

fn serve_ephemeral_client() -> JoinHandle<()> {
    thread::spawn(move || {
        let actix_system = actix_web::rt::System::new();
        actix_system.block_on(async {
            HttpServer::new(|| {
                App::new().route(
                    "/client",
                    web::get().to(|| async {
                        // Serves the example client response from the Solid OIDC primer
                        // https://solidproject.org/TR/oidc-primer
                        HttpResponse::Ok().content_type(APPLICATION_JSON).body(r#"{
                              "@context": [ "https://www.w3.org/ns/solid/oidc-context.jsonld" ],

                              "client_id": "http://127.0.0.1:10080/client",
                              "client_name": "DecentPhotos",
                              "redirect_uris": [ "https://decentphotos.example/callback", "http://localhost:3000/oidc/callback" ],
                              "post_logout_redirect_uris": [ "https://decentphotos.example/logout" ],
                              "client_uri": "https://decentphotos.example/",
                              "logo_uri": "https://decentphotos.example/logo.png",
                              "tos_uri": "https://decentphotos.example/tos.html",
                              "scope": "openid webid offline_access",
                              "grant_types": [ "refresh_token", "authorization_code" ],
                              "response_types": [ "code" ],
                              "default_max_age": 60,
                              "require_auth_time": true
                            }"#,
                        )
                    }),
                )
            })
                .bind(("127.0.0.1", 10080))
                .expect("port 10080 to be free for testing")
                .run()
                .await
                .expect("ephemeral client test http server to start") })
    })
}

#[tokio::test]
async fn test_auth_headers() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/oidc/forward_auth", backend_url);

    // make sure we get unauthorized without correct headers
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);

    // fetch a valid token and try again
    let url_token = format!("{}/oidc/token", backend_url);
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        refresh_token: None,
    };
    let res = client.post(&url_token).form(&body).send().await?;
    assert!(res.status().is_success());
    let ts = res.json::<TokenSet>().await?;

    // authorized request should return Ok and auth headers
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let headers = res.headers();
    println!("{:?}", headers);

    let value = headers.get("x-forwarded-user").unwrap().to_str().unwrap();
    assert_eq!(value, "m4PJ3TnyP32LA8hzY23deme3");

    let value = headers
        .get("x-forwarded-user-roles")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "rauthy_admin");

    // the following values are either empty or default, because the init_admin simply
    // does not have these set during login and we just make sure that the headers are there
    let value = headers
        .get("x-forwarded-user-groups")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "");

    let value = headers
        .get("x-forwarded-user-email")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "");

    let value = headers
        .get("x-forwarded-user-email-verified")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "false");

    let value = headers
        .get("x-forwarded-user-family-name")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "");

    let value = headers
        .get("x-forwarded-user-given-name")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "");

    let value = headers
        .get("x-forwarded-user-mfa")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(value, "false");

    Ok(())
}

#[tokio::test]
async fn test_token_introspection() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    // get a token to validate
    let url_token = format!("{}/oidc/token", backend_url);
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        refresh_token: None,
    };
    let res = client.post(&url_token).form(&body).send().await?;
    assert!(res.status().is_success());
    let ts = res.json::<TokenSet>().await?;

    // make sure introspection is fine
    let mut payload = TokenValidationRequest {
        token: ts.access_token.clone(),
    };
    let url = format!("{}/oidc/introspect", backend_url);

    // without proper auth should fail
    let res = client.post(&url).form(&payload).send().await?;
    assert_eq!(res.status().as_u16(), 401);

    // with valid bearer auth
    let header = format!("Bearer {}", ts.access_token);
    let res = client
        .post(&url)
        .header(AUTHORIZATION, header)
        .form(&payload)
        .send()
        .await?;
    assert!(res.status().is_success());
    let info = res.json::<TokenInfo>().await?;
    assert!(info.active);

    // with valid basic auth - client_id:client_secret
    let plain = format!("{}:{}", CLIENT_ID, CLIENT_SECRET);
    let encoded = base64_encode(plain.as_bytes());
    let header = format!("Basic {}", encoded);
    let res = client
        .post(&url)
        .header(AUTHORIZATION, &header)
        .form(&payload)
        .send()
        .await?;
    assert!(res.status().is_success());
    let info = res.json::<TokenInfo>().await?;
    assert!(info.active);

    // make sure requests with invalid tokens return inactive
    payload.token = base64_encode(b"IAmTotallyInvalid");
    let res = client
        .post(&url)
        .header(AUTHORIZATION, header)
        .form(&payload)
        .send()
        .await?;
    assert!(res.status().is_success());
    let info = res.json::<TokenInfo>().await?;
    assert!(!info.active);

    Ok(())
}

fn auth_time_from_token(id_token: &str) -> i64 {
    let (_, rest) = id_token.split_once('.').unwrap_or(("", ""));
    let (claims_b64, _) = rest.split_once('.').unwrap_or(("", ""));
    let bytes = base64_url_no_pad_decode(claims_b64).unwrap();
    let s = String::from_utf8_lossy(&bytes);
    let claims = serde_json::from_str::<serde_json::Value>(s.as_ref()).unwrap();
    let at = claims.get("auth_time").unwrap();
    at.as_i64().unwrap()
}

async fn validate_token(
    access_token: &str,
    payload: TokenValidationRequest,
) -> Result<TokenInfo, Box<dyn Error>> {
    let url_valid = format!("{}/oidc/introspect", get_backend_url());
    let res = reqwest::Client::new()
        .post(&url_valid)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .form(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let info = res.json::<TokenInfo>().await.unwrap();
    assert!(info.active);
    Ok(info)
}
