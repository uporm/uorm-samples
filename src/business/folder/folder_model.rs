use serde::{Deserialize, Serialize};
use validator::Validate;
use serde_with::{serde_as, DisplayFromStr};


#[serde_as]
#[derive(Debug, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub children: Vec<Folder>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolder {
    pub ids: Option<i64>,
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub parent_id: i32,
    #[validate(required)]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolder {
    pub id: String,
    #[validate(required)]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MoveFolder {
    pub parent_id : String,
    #[validate(range(min = 1))]
    pub seq: i32,
}