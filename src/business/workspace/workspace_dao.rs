use uorm::{sql, Param};

#[derive(Param)]
pub struct Workspace {
    pub tenant_id: u64,
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[sql("workspace")]
pub struct WorkspaceDao;

impl WorkspaceDao {
    #[sql("insert")]
    pub async fn insert(workspace: &Workspace) -> uorm::Result<()> {
        exec!()
    }

    #[sql("update")]
    pub async fn update(workspace: Workspace) -> uorm::Result<()> {
        exec!()
    }

    #[sql("delete")]
    pub async fn delete(tenant_id: u64, id: u64) -> uorm::Result<()> {
        exec!()
    }

    #[sql("list")]
    pub async fn list(tenant_id: u64) -> uorm::Result<Vec<Workspace>> {
        exec!()
    }

    #[sql("get")]
    pub async fn get(tenant_id: u64, id: u64) -> uorm::Result<Option<Workspace>> {
        exec!()
    }
}
