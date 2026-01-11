use uorm::{Param, sql};

#[derive(Param)]
pub struct Tag {
    pub id: u64,
    pub tenant_id: u64,
    pub workspace_id: u64,
    pub tag_type: i32,
    pub name: String,
}

#[sql("workspace_tag")]
pub struct TagDao;

impl TagDao {
    #[sql("insert")]
    pub async fn insert(tag: &Tag) -> uorm::Result<()> {
        exec!()
    }

    #[sql("update")]
    pub async fn update(tag: &Tag) -> uorm::Result<()> {
        exec!()
    }

    #[sql("delete")]
    pub async fn delete(
        tenant_id: u64,
        workspace_id: u64,
        id: u64,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("get")]
    pub async fn get(
        tenant_id: u64,
        workspace_id: u64,
        id: u64,
    ) -> uorm::Result<Option<Tag>> {
        exec!()
    }

    #[sql("list")]
    pub async fn list(tenant_id: u64, workspace_id: u64, tag_type: i32) -> uorm::Result<Vec<Tag>> {
        exec!()
    }
}
