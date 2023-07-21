use async_graphql::{Context, CustomDirective, Directive, ResolveFut, ServerResult, Value};

struct AuthDirective;

#[async_trait::async_trait]
impl CustomDirective for AuthDirective {
    async fn resolve_field(
        &self,
        _ctx: &Context<'_>,
        next: ResolveFut<'_>,
    ) -> ServerResult<Option<Value>> {
        next.await
    }
}

#[Directive(location = "field")]
pub fn auth() -> impl CustomDirective {
    AuthDirective
}
