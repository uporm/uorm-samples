use crate::web::ts_str::to_number;
use crate::web::ts_str::vec_to_number;
use crate::business::app::app_dao::{App, AppVersion};
use crate::utils::id::Id;
use crate::web::ts_str::to_str;
use crate::web::ts_str::vec_to_str;
use serde::{Deserialize, Serialize};
use serde_json;
use uorm::Param;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppCreateReq {
    #[serde(deserialize_with = "to_number")]
    pub folder_id: u64,
    pub app_type: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<(u64, u64, AppCreateReq)> for App {
    fn from((tenant_id, workspace_id, req): (u64, u64, AppCreateReq)) -> Self {
        Self {
            id: Id::next_id().unwrap_or_default(),
            tenant_id,
            workspace_id,
            folder_id: req.folder_id,
            app_type: req.app_type,
            name: req.name,
            description: req.description,
            tags: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateReq {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSpecUpdateReq {
    pub spec: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTagUpdateReq {
    #[serde(deserialize_with = "vec_to_number")]
    pub tag_ids: Vec<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppCloneReq {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVersionReq {
    pub version: String,
    pub description: Option<String>,
}

impl From<(App, AppVersionReq, String)> for AppVersion {
    fn from((app, req, spec): (App, AppVersionReq, String)) -> Self {
        let (major, minor, patch, pre_release) = parse_version(&req.version);
        Self {
            id: Id::next_id().unwrap_or_default(),
            tenant_id: app.tenant_id,
            workspace_id: app.workspace_id,
            app_id: app.id,
            version: req.version,
            major,
            minor,
            patch,
            pre_release,
            spec: Some(spec),
            description: req.description,
            is_latest: true,
        }
    }
}

fn parse_version(version: &str) -> (Option<i32>, Option<i32>, Option<i32>, Option<String>) {
    let (main, pre) = match version.find('-') {
        Some(idx) => (&version[..idx], Some(version[idx + 1..].to_string())),
        None => (version, None),
    };
    let parts: Vec<&str> = main.split('.').collect();
    let major = parts.get(0).and_then(|s| s.parse().ok());
    let minor = parts.get(1).and_then(|s| s.parse().ok());
    let patch = parts.get(2).and_then(|s| s.parse().ok());
    (major, minor, patch, pre)
}

#[derive(Deserialize, Serialize, Param, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppReq {
    pub folder_id: Option<u64>,
    pub app_type: Option<i32>,
    pub name: Option<String>,
    pub tag_ids: Option<Vec<u64>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppResp {
    #[serde(serialize_with = "to_str")]
    pub id: u64,
    #[serde(serialize_with = "to_str")]
    pub folder_id: u64,
    pub app_type: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(serialize_with = "vec_to_str")]
    pub tag_ids: Vec<u64>,
}

impl From<App> for AppResp {
    fn from(app: App) -> Self {
        let tag_ids: Vec<u64> = app
            .tags
            .as_ref()
            .and_then(|t| serde_json::from_str(t).ok())
            .unwrap_or_default();

        Self {
            id: app.id,
            folder_id: app.folder_id,
            app_type: app.app_type,
            name: app.name,
            description: app.description,
            tag_ids,
        }
    }
}
