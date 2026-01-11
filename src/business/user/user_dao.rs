use uorm::error::DbError;
use uorm::{sql, Param};

#[derive(Param)]
pub struct User {
    pub id: u64,
    pub tenant_id: u64,
    pub name: String,
    pub email: String,
    pub passwd: Option<String>,
    pub owner: bool,
    pub description: Option<String>,
}

#[sql("user")]
pub struct UserDao;

impl UserDao {
    #[sql("insert")]
    pub async fn insert(user: User) -> Result<i64, DbError> {
        exec!()
    }

    #[sql("list")]
    pub async fn list(tenant_id: u64) -> uorm::Result<Vec<User>> {
        exec!()
    }

    #[sql("get")]
    pub async fn get(email: &String, passwd: &String) -> uorm::Result<Option<User>> {
        exec!()
    }
}
