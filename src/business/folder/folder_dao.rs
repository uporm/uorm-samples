use crate::business::folder::folder_model::CreateFolder;
use uorm::error::DbError;
use uorm::sql;

#[sql("folder")]
pub struct FolderDao;

impl FolderDao {
    #[sql("insert")]
    pub async fn insert(folder: &mut CreateFolder) -> Result<i32, DbError> {
        exec!()
    }
}
