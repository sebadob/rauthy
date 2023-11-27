use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::HttpRequest;
use rauthy_client::principal::PrincipalOidc;

/// This is the same as `principal_opt_from_req`, but it just returns a Result to make extracting
/// the principal in multiple places nicer.
pub async fn principal_from_req(req: &HttpRequest) -> Result<PrincipalOidc, actix_web::Error> {
    match principal_opt_from_req(req).await {
        Some(p) => Ok(p),
        None => Err(ErrorUnauthorized("Unauthorized")),
    }
}

/// This is our custom / generic helper function to extract the principal.
/// Depending in your use-case, you can do this in many ways. For instance, the actix-web and
/// axum features provide FromRequest extractors, which is basically the same.
/// However, for the sake of this generic example, we do it this way.
///
/// In this example, we assume the access token will be provided as the Bearer token
/// in the Authorization header, but that is totally up to you.
pub async fn principal_opt_from_req(req: &HttpRequest) -> Option<PrincipalOidc> {
    let value = req.headers().get(AUTHORIZATION)?;

    // split off the `Bearer ` part from the value
    let (_bearer, access_token) = value
        .to_str()
        .unwrap_or_default()
        .split_once("Bearer ")
        .unwrap_or_default();

    if access_token.is_empty() {
        None
    } else {
        // PrincipalOidc::from_token_validated() is the important part.
        // This will build and validate the principal from a given raw JWT access token.
        match PrincipalOidc::from_token_validated(access_token).await {
            Ok(p) => {
                // This just makes sure, that at least the user_claim is satisfied.
                // You might skip this, if just any user from Rauthy should have access
                // to this app. no matter what groups / roles he is assigned to.
                if p.is_user {
                    Some(p)
                } else {
                    None
                }
            }
            Err(_err) => None,
        }
    }
}
