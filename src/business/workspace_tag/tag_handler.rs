use crate::business::workspace_tag::tag_dao::TagDao;
use crate::core::code::Code;
use crate::models::context::Context;
use crate::models::workspace_tag::{TagCreateReq, TagResp, TagReq, TagUpdateReq};
use crate::r;
use crate::web::error::WebError;
use crate::web::extract::Json;
use crate::web::r::R;
use axum::extract::{Path, Query};

// 查询标签列表
pub async fn list_tags(ctx: Context, Query(req): Query<TagReq>) -> R<Vec<TagResp>> {
    let tags = r!(TagDao::list(ctx.tenant_id, ctx.workspace_id, req.tag_type).await);

    let tags: Vec<TagResp> = tags.into_iter().map(Into::into).collect();
    R::ok(tags)
}

// 创建标签
pub async fn create_tag(
    ctx: Context,
    Json(req): Json<TagCreateReq>,
) -> R<String> {
    let tag = (ctx.tenant_id, ctx.workspace_id, req).into();
    r!(TagDao::insert(&tag).await);
    R::ok(tag.id.to_string())
}

// 更新标签
pub async fn update_tag(
    ctx: Context,
    Path(id): Path<u64>,
    Json(req): Json<TagUpdateReq>,
) -> R<()> {
    let mut tag = match r!(TagDao::get(ctx.tenant_id, ctx.workspace_id, id).await) {
        Some(tag) => tag,
        None => return R::err(WebError::Biz(Code::TagNotExist.into())),
    };

    tag.name = req.name;
    r!(TagDao::update(&tag).await);
    R::void()
}

// 删除标签
pub async fn delete_tag(ctx: Context, Path(id): Path<u64>) -> R<()> {
    r!(TagDao::delete(ctx.tenant_id, ctx.workspace_id, id).await);
    R::void()
}
