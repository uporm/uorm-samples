use crate::web::ts_str::to_number;
use crate::business::folder::folder_dao::Folder;
use crate::web::ts_str::to_str;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FolderResp {
    #[serde(serialize_with = "to_str")]
    pub id: i32,
    #[serde(serialize_with = "to_str")]
    pub parent_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub seq: i32,
    pub children: Vec<FolderResp>,
}

impl From<Folder> for FolderResp {
    fn from(folder: Folder) -> Self {
        Self {
            id: folder.id,
            parent_id: folder.parent_id,
            name: folder.name,
            description: folder.description,
            seq: folder.seq,
            children: vec![],
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderReq {
    #[serde(deserialize_with = "to_number")]
    pub parent_id: i32,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderReq {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MoveFolderReq {
    #[serde(deserialize_with = "to_number")]
    pub parent_id: i32,
    pub seq: i32,
}
