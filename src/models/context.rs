#[derive(Debug, Clone)]
pub struct Context {
    pub tenant_id: u64,
    pub user_id: u64,
    pub workspace_id: u64,
}

// Axum extractor for Context
impl<S> axum::extract::FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Context>()
            .cloned()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)
    }
}
