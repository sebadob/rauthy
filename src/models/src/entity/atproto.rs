use atrium_identity::{
    did::CommonDidResolver,
    handle::{AtprotoHandleResolver, DnsTxtResolver as DnsTxtResolverTrait},
};
use atrium_oauth_client::{DefaultHttpClient, OAuthClient};
use hickory_resolver::{proto::rr::rdata::TXT, TokioAsyncResolver};

use crate::database::DB;

type AtprotoClient = OAuthClient<
    DB,
    DB,
    CommonDidResolver<DefaultHttpClient>,
    AtprotoHandleResolver<DnsTxtResolver, DefaultHttpClient>,
>;

struct DnsTxtResolver {
    resolver: TokioAsyncResolver,
}

impl Default for DnsTxtResolver {
    fn default() -> Self {
        Self {
            resolver: TokioAsyncResolver::tokio_from_system_conf()
                .expect("failed to create resolver"),
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
