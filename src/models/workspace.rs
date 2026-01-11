use crate::business::workspace::workspace_dao::Workspace;
use crate::utils::id::Id;
use crate::web::ts_str::to_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceReq {
    pub name: String,
    pub description: Option<String>,
}

impl From<(u64, WorkspaceReq)> for Workspace {
    fn from((tenant_id, req): (u64, WorkspaceReq)) -> Self {
        Self {
            id: Id::next_id().unwrap_or_default(),
            tenant_id,
            name: req.name,
            description: req.description,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResp {
    #[serde(serialize_with = "to_str")]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub selected: bool,
}

impl From<Workspace> for WorkspaceResp {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id,
            name: workspace.name,
            description: workspace.description,
            selected: false,
        }
    }
}
