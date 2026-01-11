use crate::business::cache::workspace_cache;
use crate::business::workspace::workspace_consumer::WorkspaceConsumer;
use crate::business::workspace::workspace_dao::{Workspace, WorkspaceDao};
use crate::core::code::Code;
use crate::models::context::Context;
use crate::models::workspace::{WorkspaceReq, WorkspaceResp};
use crate::r;
use crate::web::error::WebError;
use crate::web::extract::Json;
use crate::web::r::R;
use axum::extract::Path;
use uorm::transaction;

// 查询工作空间列表
pub async fn list_workspaces(ctx: Context) -> R<Vec<WorkspaceResp>> {
    let workspaces = r!(WorkspaceDao::list(ctx.tenant_id).await);
    let mut workspaces: Vec<WorkspaceResp> = workspaces.into_iter().map(Into::into).collect();

    // 标记选中的工作空间
    let cached_id = workspace_cache::get_workspace_id(ctx.user_id).await;
    let selected_id = cached_id.or_else(|| workspaces.first().map(|w| w.id));

    if let Some(id) = selected_id {
        if let Some(ws) = workspaces.iter_mut().find(|w| w.id == id) {
            ws.selected = true;
            // 如果没有缓存ID，说明是首次访问，需要缓存第一个工作空间
            if cached_id.is_none() {
                workspace_cache::switch_workspace(ctx.user_id, id).await;
            }
        }
    }

    R::ok(workspaces)
}

// 创建工作空间
#[transaction]
pub async fn create_workspace(ctx: Context, Json(req): Json<WorkspaceReq>) -> R<()> {
    let workspace = (ctx.tenant_id, req).into();
    r!(WorkspaceDao::insert(&workspace).await);
    r!(WorkspaceConsumer::start_dispatching(ctx.tenant_id, workspace.id).await);
    R::void()
}

// 更新工作空间
pub async fn update_workspace(
    ctx: Context,
    Path(id): Path<u64>,
    Json(req): Json<WorkspaceReq>,
) -> R<()> {
    let mut workspace = r!(check_workspace_exists(ctx.tenant_id, id).await);
    workspace.name = req.name;
    workspace.description = req.description;
    r!(WorkspaceDao::update(workspace).await);
    R::void()
}

// 删除工作空间
pub async fn delete_workspace(ctx: Context, Path(id): Path<u64>) -> R<()> {
    if ctx.workspace_id == id {
        return R::err(WebError::Biz(Code::WorkspaceCurrentCannotDelete.into()));
    }
    r!(WorkspaceDao::delete(ctx.tenant_id, id).await);
    R::void()
}

// 切换工作空间
pub async fn switch_workspace(ctx: Context, Path(id): Path<u64>) -> R<()> {
    r!(check_workspace_exists(ctx.tenant_id, id).await);
    workspace_cache::switch_workspace(ctx.user_id, id).await;
    R::void()
}

// 检查工作空间是否存在
async fn check_workspace_exists(tenant_id: u64, workspace_id: u64) -> Result<Workspace, WebError> {
    WorkspaceDao::get(tenant_id, workspace_id)
        .await?
        .ok_or_else(|| WebError::Biz(Code::WorkspaceNotExist.into()))
}
