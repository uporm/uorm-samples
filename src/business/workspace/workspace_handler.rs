use crate::business::workspace::workspace_dao::{Workspace, WorkspaceDao};
use crate::models::context::Context;
use crate::models::workspace::{WorkspaceReq, WorkspaceResp};
use crate::r;
use crate::web::extract::Json;
use crate::web::r::R;
use axum::extract::Path;

// 查询工作空间列表
pub async fn list_workspaces(ctx: Context) -> R<Vec<WorkspaceResp>> {
    let workspaces = r!(WorkspaceDao::list(ctx.tenant_id).await);
    let workspaces = workspaces
        .into_iter()
        .map(|workspace| workspace.into())
        .collect::<Vec<WorkspaceResp>>();
    R::ok(workspaces)
}

// 创建工作空间
pub async fn create_workspace(ctx: Context, Json(req): Json<WorkspaceReq>) -> R<()> {
    let max_key = r!(WorkspaceDao::get_max_key(ctx.tenant_id).await);
    let next_key = max_key.map(|key| key + 1).unwrap_or(0);
    let workspace = Workspace::new(ctx.tenant_id, next_key, req.name, req.description);
    r!(WorkspaceDao::insert(workspace).await);
    R::void()
}

// 更新工作空间
pub async fn update_workspace(
    ctx: Context,
    Path(key): Path<i32>,
    Json(req): Json<WorkspaceReq>,
) -> R<()> {
    let workspace = Workspace::new(ctx.tenant_id, key, req.name, req.description);
    r!(WorkspaceDao::update(workspace).await);
    R::void()
}

// 删除工作空间
pub async fn delete_workspace(ctx: Context, Path(key): Path<i32>) -> R<()> {
    r!(WorkspaceDao::delete(ctx.tenant_id, key).await);
    R::void()
}
