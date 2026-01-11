use crate::business::cache::workspace_cache;
use crate::models::context::Context;
use crate::web::r::R;
use axum::extract::Request;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use rust_i18n::t;
use crate::core::code::Code;

/// 认证中间件：从 header 中提取租户 ID、用户 ID、工作空间 ID
pub async fn handle_auth(mut req: Request, next: Next) -> Response {
    let context = extract_context(req.headers())
        .await
        .map_err(|e| e.into_response());
    if let Err(e) = context {
        return e;
    }

    let context = context.unwrap();
    
    // 检查是否为 workspace 模块的请求
    let path = req.uri().path();
    let is_workspace_request = path.starts_with("/uvwa/api/workspaces");
    
    // 如果不是 workspace 请求且 workspace_id 为 0，返回错误
    if !is_workspace_request && context.workspace_id == 0 {
        return workspace_not_selected_error().into_response();
    }

    // 将上下文放入请求扩展中供后续处理器使用
    req.extensions_mut().insert(context);
    next.run(req).await
}

/// 从 header 中提取上下文信息
async fn extract_context(headers: &HeaderMap) -> Result<Context, R<()>> {
    let tenant_id = extract_u64_header(headers, "x-tenant-id").ok_or_else(unauthorized_error)?;

    let user_id = extract_u64_header(headers, "x-user-id").ok_or_else(unauthorized_error)?;

    let workspace_id = workspace_cache::get_workspace_id(user_id).await;

    Ok(Context {
        tenant_id,
        user_id,
        workspace_id: workspace_id.unwrap_or_default(),
    })
}

fn unauthorized_error() -> R<()> {
    let code: i32 = Code::Unauthorized.into();
    let key = code.to_string();
    R {
        code,
        message: t!(&key).to_string(),
        data: None,
    }
}

fn workspace_not_selected_error() -> R<()> {
    let code: i32 = Code::WorkspaceNotSelected.into();
    let key = code.to_string();
    R {
        code,
        message: t!(&key).to_string(),
        data: None,
    }
}

/// 从 header 中提取 i32 类型的值
fn extract_u64_header(headers: &HeaderMap, key: &str) -> Option<u64> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
}
