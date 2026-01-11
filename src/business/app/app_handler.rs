use crate::business::app::app_dao::{App, AppDao};
use crate::business::workspace_folder::folder_dao::FolderDao;
use crate::core::code::Code;
use crate::models::app::{
    AppCloneReq, AppCreateReq, AppReq, AppResp, AppSpecUpdateReq, AppTagUpdateReq, AppUpdateReq,
    AppVersionReq,
};
use crate::models::context::Context;
use crate::r;
use crate::utils::id::Id;
use crate::web::error::WebError;
use crate::web::extract::Json;
use crate::web::r::R;
use axum::extract::Path;
use axum_extra::extract::Query;
use serde_json;
use uorm::transaction;
use validator::Validate;

// 查询应用列表
pub async fn list_apps(ctx: Context, Query(req): Query<AppReq>) -> R<Vec<AppResp>> {
    // 查询应用列表
    let apps = r!(AppDao::list(ctx.tenant_id, ctx.workspace_id, &req).await);
    let app_resps = apps.into_iter().map(Into::into).collect();
    R::ok(app_resps)
}

// 获取应用草稿
pub async fn get_app_spec(ctx: Context, Path(id): Path<u64>) -> R<Option<String>> {
    let spec = r!(AppDao::get_spec(ctx.tenant_id, ctx.workspace_id, id).await);
    R::ok(spec)
}
// 创建应用
pub async fn create_app(ctx: Context, Json(req): Json<AppCreateReq>) -> R<()> {
    r!(req.validate());

    // 检查所属目录是否存在
    if req.folder_id != 0 {
        let folder = r!(FolderDao::get_by_id(ctx.tenant_id, ctx.workspace_id, req.folder_id).await);
        if folder.is_none() {
            return R::err(WebError::Biz(Code::AppParentFolderNotExist.into()));
        }
    }

    let app: App = (ctx.tenant_id, ctx.workspace_id, req).into();
    r!(AppDao::insert(&app).await);
    R::void()
}

// 更新应用
pub async fn update_app(ctx: Context, Path(id): Path<u64>, Json(req): Json<AppUpdateReq>) -> R<()> {
    let exist = r!(AppDao::get_by_id(ctx.tenant_id, ctx.workspace_id, id).await);
    if exist.is_none() {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    let mut app = exist.unwrap();
    app.name = req.name;
    app.description = req.description;

    r!(AppDao::update(&app).await);
    R::void()
}

// 删除应用
#[transaction]
pub async fn delete_app(ctx: Context, Path(id): Path<u64>) -> R<()> {
    let exist = r!(AppDao::get_by_id(ctx.tenant_id, ctx.workspace_id, id).await);
    if exist.is_none() {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    r!(AppDao::delete(ctx.tenant_id, ctx.workspace_id, id).await);
    r!(AppDao::delete_versions(ctx.tenant_id, ctx.workspace_id, id).await);
    R::void()
}
// 更新应用草稿
pub async fn update_app_spec(
    ctx: Context,
    Path(id): Path<u64>,
    Json(req): Json<AppSpecUpdateReq>,
) -> R<()> {
    let exist = r!(AppDao::get_by_id(ctx.tenant_id, ctx.workspace_id, id).await);
    if exist.is_none() {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    r!(AppDao::update_spec(ctx.tenant_id, ctx.workspace_id, id, &req.spec).await);
    R::void()
}

pub async fn update_app_tags(
    ctx: Context,
    Path(id): Path<u64>,
    Json(req): Json<AppTagUpdateReq>,
) -> R<()> {
    let exist = r!(AppDao::get_by_id(ctx.tenant_id, ctx.workspace_id, id).await);
    if exist.is_none() {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    let mut app = exist.unwrap();
    app.tags = if req.tag_ids.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&req.tag_ids).unwrap_or_default())
    };
    r!(AppDao::update(&app).await);
    R::void()
}

pub async fn clone_app(ctx: Context, Path(id): Path<u64>, Json(req): Json<AppCloneReq>) -> R<()> {
    let new_app_id = Id::next_id().unwrap();

    let rows = r!(AppDao::clone_app(
        ctx.tenant_id,
        ctx.workspace_id,
        id,
        new_app_id,
        &req.name,
        &req.description
    )
    .await);

    if rows == 0 {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    R::void()
}

#[transaction]
pub async fn release_app(
    ctx: Context,
    Path(id): Path<u64>,
    Json(req): Json<AppVersionReq>,
) -> R<()> {
    let exist = r!(AppDao::get_by_id(ctx.tenant_id, ctx.workspace_id, id).await);
    if exist.is_none() {
        return R::err(WebError::Biz(Code::AppNotExist.into()));
    }

    let spec = r!(AppDao::get_spec(ctx.tenant_id, ctx.workspace_id, id).await);
    if spec.is_none() {
        return R::err(WebError::Biz(Code::AppDraftNotExist.into()));
    }

    r!(AppDao::cancel_latest(ctx.tenant_id, ctx.workspace_id, id).await);

    let version = (exist.unwrap(), req, spec.unwrap()).into();
    r!(AppDao::insert_version(&version).await);

    R::void()
}
