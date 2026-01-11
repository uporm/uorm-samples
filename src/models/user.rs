use crate::business::user::user_dao::User;
use crate::web::ts_str::to_str;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::id::Id;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUpReq {
    #[validate(email)]
    pub email: String,
}

impl From<SignUpReq> for User {
    fn from(req: SignUpReq) -> Self {
        Self {
            id: Id::next_id().unwrap(),
            tenant_id: 0,
            name: "匿名".to_string(),
            email: req.email,
            passwd: None,
            owner: true,
            description: None,
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignInReq {
    #[validate(email)]
    pub email: String,
    pub passwd: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResp {
    #[serde(serialize_with = "to_str")]
    pub id: u64,
    pub name: String,
    pub email: String,
    pub description: Option<String>,
}

impl From<User> for UserResp {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            description: user.description,
        }
    }
}
