use crate::business::workspace_folder::folder_dao::Folder;
use crate::web::ts_str::to_number;
use crate::web::ts_str::to_str;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FolderResp {
    #[serde(serialize_with = "to_str")]
    pub id: u64,
    #[serde(serialize_with = "to_str")]
    pub parent_id: u64,
    pub name: String,
    pub seq: i32,
    pub children: Vec<FolderResp>,
}

impl From<Folder> for FolderResp {
    fn from(folder: Folder) -> Self {
        Self {
            id: folder.id,
            parent_id: folder.parent_id,
            name: folder.name,
            seq: folder.seq,
            children: vec![],
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderReq {
    pub folder_type: i32,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderReq {
    #[serde(deserialize_with = "to_number")]
    pub parent_id: u64,
    #[validate(length(min = 1))]
    pub name: String,
    pub folder_type: i32,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderReq {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MoveFolderReq {
    #[serde(deserialize_with = "to_number")]
    pub parent_id: u64,
    pub seq: i32,
}
