use atrium_identity::{
    did::CommonDidResolver,
    handle::{AtprotoHandleResolver, DnsTxtResolver as DnsTxtResolverTrait},
};
use atrium_oauth::{DefaultHttpClient, OAuthClient};
use hickory_resolver::{
    Resolver, TokioResolver, config::ResolverConfig, name_server::TokioConnectionProvider,
    proto::rr::rdata::TXT,
};

use crate::database::DB;

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
