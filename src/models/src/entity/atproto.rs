use atrium_identity::handle::DnsTxtResolver as DnsTxtResolverTrait;
use hickory_resolver::{proto::rr::rdata::TXT, TokioAsyncResolver};

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
