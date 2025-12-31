use uorm::{Param, sql};

#[derive(Param)]
pub struct Workspace {
    pub tenant_id: u64,
    pub key: i32,
    pub name: String,
    pub description: Option<String>,
}

impl Workspace {
    pub fn new(tenant_id: u64, key: i32, name: String, description: Option<String>) -> Self {
        Workspace {
            tenant_id,
            key,
            name,
            description,
        }
    }
}

#[sql("workspace")]
pub struct WorkspaceDao;

impl WorkspaceDao {
    #[sql("insert")]
    pub async fn insert(workspace: Workspace) -> uorm::Result<()> {
        exec!()
    }

    #[sql("update")]
    pub async fn update(workspace: Workspace) -> uorm::Result<()> {
        exec!()
    }

    #[sql("delete")]
    pub async fn delete(tenant_id: u64, key: i32) -> uorm::Result<()> {
        exec!()
    }

    #[sql("list")]
    pub async fn list(tenant_id: u64) -> uorm::Result<Vec<Workspace>> {
        exec!()
    }

    #[sql("getMaxKey")]
    pub async fn get_max_key(tenant_id: u64) -> uorm::Result<Option<i32>> {
        exec!()
    }
}
