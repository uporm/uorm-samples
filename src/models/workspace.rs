use crate::business::workspace::workspace_dao::Workspace;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceReq {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResp {
    pub key: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<Workspace> for WorkspaceResp {
    fn from(workspace: Workspace) -> Self {
        Self {
            key: workspace.key,
            name: workspace.name,
            description: workspace.description,
        }
    }
}
