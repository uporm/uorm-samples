use crate::utils::id::Id;
use uorm::{Param, sql};

#[derive(Param, Default)]
pub struct Folder {
    pub id: u64,
    pub tenant_id: u64,
    pub workspace_id: u64,
    pub parent_id: u64,
    pub folder_type: i32,
    pub name: String,
    pub seq: i32,
}

impl Folder {
    pub fn new(tenant_id: u64, workspace_id: u64) -> Self {
        Self {
            id: Id::next_id().unwrap_or_default(),
            tenant_id,
            workspace_id,
            ..Default::default()
        }
    }
}

#[sql("workspace_folder")]
pub struct FolderDao;

impl FolderDao {
    #[sql("list")]
    pub async fn list(
        tenant_id: u64,
        workspace_id: u64,
        folder_type: i32,
    ) -> uorm::Result<Vec<Folder>> {
        exec!()
    }

    #[sql("countChildren")]
    pub async fn count_children(
        tenant_id: u64,
        workspace_id: u64,
        parent_id: u64,
    ) -> uorm::Result<i32> {
        exec!()
    }

    #[sql("getById")]
    pub async fn get_by_id(
        tenant_id: u64,
        workspace_id: u64,
        id: u64,
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
    pub async fn delete(tenant_id: u64, workspace_id: u64, id: u64) -> uorm::Result<()> {
        exec!()
    }

    #[sql("getMaxSeq")]
    pub async fn get_max_seq(
        tenant_id: u64,
        workspace_id: u64,
        parent_id: u64,
    ) -> uorm::Result<Option<i32>> {
        exec!()
    }

    #[sql("shiftSeqOnInsert")]
    pub async fn shift_seq(
        tenant_id: u64,
        workspace_id: u64,
        parent_id: u64,
        from_seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("compressSeqOnRemove")]
    pub async fn compress_seq(
        tenant_id: u64,
        workspace_id: u64,
        parent_id: u64,
        from_seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }

    #[sql("updateParentAndSeq")]
    pub async fn update_parent_and_seq(
        tenant_id: u64,
        workspace_id: u64,
        id: u64,
        parent_id: u64,
        seq: i32,
    ) -> uorm::Result<()> {
        exec!()
    }
}
