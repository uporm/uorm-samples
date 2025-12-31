use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use crate::web::code::Code;
use crate::web::r::R;

pub async fn not_found(uri: Uri) -> impl IntoResponse {
    let r: R<()> = R {
        code: Code::NotFound.as_i32(),
        message: format!("Route not found: {}", uri),
        data: None,
    };
    r
}

pub async fn method_not_allowed(uri: Uri, method: Method) -> impl IntoResponse {
    let r: R<()> = R {
        code: Code::MethodNotAllowed.as_i32(),
        message: format!("Method {} not allowed for {}", method, uri),
        data: None,
    };
    r
}
