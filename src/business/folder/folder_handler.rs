use crate::business::folder::folder_dao::{Folder, FolderDao};
use crate::models::context::Context;
use crate::models::folder::{CreateFolderReq, FolderResp, MoveFolderReq, UpdateFolderReq};
use crate::r;
use crate::web::code::Code;
use crate::web::error::WebError;
use crate::web::extract::Json;
use crate::web::r::R;
use axum::extract::Path;
use std::collections::HashMap;
use validator::Validate;

// 查询文件夹树
pub async fn get_folder_tree(ctx: Context, Path(r#type): Path<i32>) -> R<Vec<FolderResp>> {
    let tenant_id = ctx.tenant_id;
    let workspace_key = ctx.workspace_key as i32;

    let mut folders = r!(FolderDao::list(tenant_id, workspace_key, r#type).await);

    // If no folders, create default one
    if folders.is_empty() {
        let folder = Folder::new(tenant_id, workspace_key, r#type);
        r!(FolderDao::insert(&folder).await);
        folders = r!(FolderDao::list(tenant_id, workspace_key, r#type).await);
    }

    let folder_resps: Vec<FolderResp> = folders.into_iter().map(|f| f.into()).collect();
    let tree = build_folder_tree(folder_resps);
    R::ok(tree)
}

fn build_folder_tree(folders: Vec<FolderResp>) -> Vec<FolderResp> {
    let mut map_by_parent: HashMap<i32, Vec<FolderResp>> = HashMap::new();
    for f in folders {
        map_by_parent.entry(f.parent_id).or_default().push(f);
    }
    
    build_recursive(0, &mut map_by_parent)
}

fn build_recursive(parent_id: i32, map: &mut HashMap<i32, Vec<FolderResp>>) -> Vec<FolderResp> {
    let mut children = map.remove(&parent_id).unwrap_or_default();
    
    for child in &mut children {
        child.children = build_recursive(child.id, map);
    }
    children
}

// 创建文件夹
pub async fn create_folder(
    ctx: Context,
    Path(r#type): Path<i32>,
    Json(req): Json<CreateFolderReq>,
) -> R<String> {
    r!(req.validate());
    let tenant_id = ctx.tenant_id;
    let workspace_key = ctx.workspace_key as i32;

    // Check parent
    if req.parent_id != 0 {
        let parent = r!(FolderDao::get_by_id(tenant_id, workspace_key, req.parent_id).await);
        if parent.is_none() {
            return R::err(WebError::Biz(Code::FolderParentNotExist.as_i32(), vec![]));
        }
    }

    let max_seq = r!(FolderDao::get_max_seq(tenant_id, workspace_key, req.parent_id).await);
    let seq = max_seq.unwrap_or(0) + 1;

    let mut folder = Folder::new(tenant_id, workspace_key, r#type);
    folder.parent_id(req.parent_id)
        .name(req.name)
        .seq(seq)
        .description(req.description);

    let id = r!(FolderDao::insert(&folder).await);
    R::ok(id.to_string())
}

// 更新文件夹
pub async fn update_folder(
    ctx: Context,
    Path((_type, id)): Path<(i32, i32)>, 
    Json(req): Json<UpdateFolderReq>,
) -> R<()> {
    r!(req.validate());
    let tenant_id = ctx.tenant_id;
    let workspace_key = ctx.workspace_key as i32;

    let Some(mut folder) = r!(FolderDao::get_by_id(tenant_id, workspace_key, id).await) else {
        return R::err(WebError::Biz(Code::FolderNotExist.as_i32(), vec![]));
    };

    folder.name = req.name;
    folder.description = req.description;
    r!(FolderDao::update(&folder).await);
    R::void()
}

// 删除文件夹
pub async fn delete_folder(ctx: Context, Path((_type, id)): Path<(i32, i32)>) -> R<()> {
    let tenant_id = ctx.tenant_id;
    let workspace_key = ctx.workspace_key as i32;

    let Some(folder) = r!(FolderDao::get_by_id(tenant_id, workspace_key, id).await) else {
        return R::err(WebError::Biz(Code::FolderNotExist.as_i32(), vec![]));
    };

    // Check children
    let count = r!(FolderDao::count_children(tenant_id, workspace_key, id).await);
    if count > 0 {
        return R::err(WebError::Biz(Code::FolderNotEmpty.as_i32(), vec![]));
    }

    r!(FolderDao::delete(tenant_id, workspace_key, id).await);
    r!(FolderDao::compress_seq_on_remove(tenant_id, workspace_key, folder.parent_id, folder.seq).await);
    R::void()
}

// 移动文件夹
pub async fn move_folder(
    ctx: Context,
    Path((_type, id)): Path<(i32, i32)>,
    Json(req): Json<MoveFolderReq>,
) -> R<()> {
    r!(req.validate());
    let tenant_id = ctx.tenant_id;
    let workspace_key = ctx.workspace_key as i32;

    if id == req.parent_id {
        return R::err(WebError::Biz(Code::FolderMoveToSelf.as_i32(), vec![]));
    }

    let Some(folder) = r!(FolderDao::get_by_id(tenant_id, workspace_key, id).await) else {
        return R::err(WebError::Biz(Code::FolderNotExist.as_i32(), vec![]));
    };

    if req.parent_id != 0 {
        let parent = r!(FolderDao::get_by_id(tenant_id, workspace_key, req.parent_id).await);
        if parent.is_none() {
            return R::err(WebError::Biz(Code::FolderParentNotExist.as_i32(), vec![]));
        }
    }

    r!(FolderDao::compress_seq_on_remove(tenant_id, workspace_key, folder.parent_id, folder.seq).await);
    r!(FolderDao::shift_seq_on_insert(tenant_id, workspace_key, req.parent_id, req.seq).await);
    r!(FolderDao::update_parent_and_seq(tenant_id, workspace_key, id, req.parent_id, req.seq).await);
    R::void()
}
