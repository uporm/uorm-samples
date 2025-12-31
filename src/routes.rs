use crate::business::folder::folder_handler;
use crate::business::user::user_handler;
use crate::business::workspace::workspace_handler;
use crate::web::middleware::auth::handle_auth;
use axum::middleware;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router {
    // 公开路由
    let public_routes = Router::new()
        .nest("/nowa/users", public_user_routes());

    // 受保护路由
    let protected_routes = Router::new()
        .nest("/nowa", user_routes())
        .nest("/nowa", workspace_routes())
        .nest("/nowa", folder_routes())
        .layer(middleware::from_fn(handle_auth));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}

// user routes
fn public_user_routes() -> Router {
    Router::new()
        .route("/users/sign-up", post(user_handler::sign_up))
        .route("/users/sign-in", post(user_handler::sign_in))
}

fn user_routes() -> Router {
    Router::new()
        .route("/users", get(user_handler::list_users))
}

// Workspace routes
fn workspace_routes() -> Router {
    Router::new()
        .route("/workspaces", get(workspace_handler::list_workspaces))
        .route("/workspaces", post(workspace_handler::create_workspace))
        .route("/workspaces/{key}", put(workspace_handler::update_workspace))
        .route("/workspaces/{key}", delete(workspace_handler::delete_workspace))
}

// Folder routes
fn folder_routes() -> Router {
    Router::new()
        .route("/folders/{type}", get(folder_handler::get_folder_tree))
        .route("/folders/{type}", post(folder_handler::create_folder))
        .route("/folders/{type}/{id}", put(folder_handler::update_folder))
        .route("/folders/{type}/{id}", delete(folder_handler::delete_folder))
        .route("/folders/{type}/{id}/move", put(folder_handler::move_folder))
}