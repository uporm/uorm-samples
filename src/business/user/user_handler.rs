use crate::business::user::user_dao::{User, UserDao};
use crate::models::context::Context;
use crate::models::user::{SignInReq, SignUpReq, UserResp};
use crate::r;
use crate::utils::id::Id;
use crate::core::code::Code;
use crate::web::error::WebError::Biz;
use crate::web::extract::Json;
use crate::web::r::R;
use validator::Validate;

// 注册用户
pub async fn sign_up(Json(req): Json<SignUpReq>) -> R<()> {
    r!(req.validate());
    let tenant_id = r!(Id::next_id());
    let mut user: User = req.into();
    user.tenant_id = tenant_id;
    r!(UserDao::insert(user).await);
    R::void()
}

// 登录用户
pub async fn sign_in(Json(user): Json<SignInReq>) -> R<()> {
    r!(user.validate());
    let user = r!(UserDao::get(&user.email, &user.passwd).await);
    if user.is_none() {
        return R::err(Biz(Code::Unauthorized.into()));
    }
    R::void()
}

// 列出用户
pub async fn list_users(ctx: Context) -> R<Vec<UserResp>> {
    let users = r!(UserDao::list(ctx.tenant_id).await);
    let users = users
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<UserResp>>();
    R::ok(users)
}
