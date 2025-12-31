use uorm::{Param, sql};

#[derive(Param, Default)]
pub struct Folder {
    pub id: i32,
    pub tenant_id: u64,
    pub workspace_key: i32,
    pub parent_id: i32,
    pub type_: i32,
    pub name: String,
    pub seq: i32,
    pub description: Option<String>,
}

impl Folder {
    pub fn new(tenant_id: u64, workspace_key: i32, type_: i32) -> Self {
        Self {
            id: 0,
            tenant_id,
            workspace_key,
            parent_id: 0,
            type_,
            name: "默认目录".to_string(),
            seq: 0,
            description: Some("系统自动创建的默认目录".to_string()),
        }
    }

    pub fn parent_id(&mut self, parent_id: i32) -> &mut Self {
        self.parent_id = parent_id;
        self
    }
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn seq(&mut self, seq: i32) -> &mut Self {
        self.seq = seq;
        self
    }
    pub fn description(&mut self, description: Option<String>) -> &mut Self {
        self.description = description;
        self
    }
}

#[sql("folder")]
pub struct FolderDao;

impl FolderDao {
    #[sql("list")]
    pub async fn list(
        tenant_id: u64,
        workspace_key: i32,
        type_: i32,
    ) -> uorm::Result<Vec<Folder>> {
        exec!()
    }

    #[sql("countChildren")]
    pub async fn count_children(
        tenant_id: u64,
        workspace_key: i32,
        parent_id: i32,
    ) -> uorm::Result<i32> {
        exec!()
    }

    #[sql("getById")]
    pub async fn get_by_id(
        tenant_id: u64,
        workspace_key: i32,
        id: i32,
    ) -> uorm::Result<Option<Folder>> {
        exec!()
    }

    #[sql("insert")]
    pub async fn insert(folder: &Folder) -> uorm::Result<u64> {
        exec!()
    }

    #[sql("update")]
    pub async fn update(folder: &Folder) -> uorm::Result<()> {
        exec!()
    }

    #[sql("delete")]
    pub async fn delete(tenant_id: u64, workspace_key: i32, id: i32) -> uorm::Result<()> {
        exec!()
    }

    #[sql("getMaxSeq")]
    pub async fn get_max_seq(
        tenant_id: u64,
        workspace_key: i32,
        parent_id: i32,
    ) -> uorm::Result<Option<i32>> {
        exec!()
    }

    #[sql("shiftSeqOnInsert")]
    pub async fn shift_seq_on_insert(
        tenant_id: u64,
        workspace_key: i32,
        parent_id: i32,
        from_seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("compressSeqOnRemove")]
    pub async fn compress_seq_on_remove(
        tenant_id: u64,
        workspace_key: i32,
        parent_id: i32,
        from_seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("updateParentAndSeq")]
    pub async fn update_parent_and_seq(
        tenant_id: u64,
        workspace_key: i32,
        id: i32,
        parent_id: i32,
        seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }
}
