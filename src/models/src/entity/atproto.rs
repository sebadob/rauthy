use actix_web::{cookie::Cookie, http::header::HeaderValue, web};
use atrium_identity::{
    did::CommonDidResolver,
    handle::{AtprotoHandleResolver, DnsTxtResolver as DnsTxtResolverTrait},
};
use hickory_resolver::{
    Resolver, TokioResolver, config::ResolverConfig, name_server::TokioConnectionProvider,
    proto::rr::rdata::TXT,
};
use rauthy_api_types::atproto;
use rauthy_error::ErrorResponse;

use crate::{app_state::AppState, database::DB};

pub type AtprotoClient = OAuthClient<
    DB,
    DB,
    CommonDidResolver<DefaultHttpClient>,
    AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
>;

pub struct DnsTxtResolver {
    resolver: TokioResolver,
}

impl Default for DnsTxtResolver {
    fn default() -> Self {
        Self {
            resolver: Resolver::builder_with_config(
                ResolverConfig::default(),
                TokioConnectionProvider::default(),
            )
            .build(),
        }
    }
}

impl DnsTxtResolverTrait for DnsTxtResolver {
    async fn resolve(
        &self,
        query: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let txt_lookup = self.resolver.txt_lookup(query).await?;
        Ok(txt_lookup.iter().map(TXT::to_string).collect())
    }
}

pub trait AtprotoCallback {
    async fn login_start(
        data: &web::Data<AppState>,
        payload: atproto::LoginRequest,
    ) -> Result<(Cookie<'_>, String, HeaderValue), ErrorResponse>;
}
