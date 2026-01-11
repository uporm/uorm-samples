use crate::models::app::AppReq;
use uorm::{Param, sql};

#[derive(Param)]
pub struct App {
    pub id: u64,
    pub tenant_id: u64,
    pub workspace_id: u64,
    pub folder_id: u64,
    pub app_type: i32,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Param)]
pub struct AppVersion {
    pub id: u64,
    pub tenant_id: u64,
    pub workspace_id: u64,
    pub app_id: u64,
    pub version: String,
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
    pub pre_release: Option<String>,
    pub spec: Option<String>,
    pub description: Option<String>,
    pub is_latest: bool,
}

#[sql("app")]
pub struct AppDao;

impl AppDao {
    #[sql("list")]
    pub async fn list(tenant_id: u64, workspace_id: u64, query: &AppReq) -> uorm::Result<Vec<App>> {
        exec!()
    }

    #[sql("getById")]
    pub async fn get_by_id(
        tenant_id: u64,
        workspace_id: u64,
        id: u64,
    ) -> uorm::Result<Option<App>> {
        exec!()
    }

    #[sql("insert")]
    pub async fn insert(app: &App) -> uorm::Result<u64> {
        exec!()
    }

    #[sql("update")]
    pub async fn update(app: &App) -> uorm::Result<()> {
        exec!()
    }

    #[sql("delete")]
    pub async fn delete(tenant_id: u64, workspace_id: u64, id: u64) -> uorm::Result<()> {
        exec!()
    }

    #[sql("deleteByFolderId")]
    pub async fn delete_by_folder_id(
        tenant_id: u64,
        workspace_id: u64,
        folder_id: u64,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("clone")]
    pub async fn clone_app(
        tenant_id: u64,
        workspace_id: u64,
        old_id: u64,
        new_id: u64,
        name: &str,
        description: &Option<String>,
    ) -> uorm::Result<u64> {
        exec!()
    }

    #[sql("getSpec")]
    pub async fn get_spec(
        tenant_id: u64,
        workspace_id: u64,
        app_id: u64,
    ) -> uorm::Result<Option<String>> {
        exec!()
    }

    #[sql("updateSpec")]
    pub async fn update_spec(
        tenant_id: u64,
        workspace_id: u64,
        app_id: u64,
        spec: &String,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("cancelLatest")]
    pub async fn cancel_latest(tenant_id: u64, workspace_id: u64, app_id: u64) -> uorm::Result<()> {
        exec!()
    }

    #[sql("insertVersion")]
    pub async fn insert_version(version: &AppVersion) -> uorm::Result<u64> {
        exec!()
    }

    #[sql("deleteVersions")]
    pub async fn delete_versions(
        tenant_id: u64,
        workspace_id: u64,
        app_id: u64,
    ) -> uorm::Result<()> {
        exec!()
    }
}
