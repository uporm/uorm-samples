use crate::utils::id::Id;
use crate::web::ts_str::to_str;
use serde::Serialize;
use uorm::error::DbError;
use uorm::{sql, Param};

#[derive(Debug, Param, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(serialize_with = "to_str")]
    pub id: u64,
    pub tenant_id: u64,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub owner: bool,
    pub description: Option<String>,
}

impl User {
    pub fn new(tenant_id: u64, email: String, owner: bool) -> Self {
        Self {
            id: Id::next_id().unwrap(),
            tenant_id,
            name: "uflow".to_string(),
            email,
            password: None,
            owner,
            description: None,
        }
    }
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
