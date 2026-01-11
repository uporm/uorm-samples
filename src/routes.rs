use crate::business::app::app_handler;
use crate::business::user::user_handler;
use crate::business::workspace::workspace_handler;
use crate::business::workspace_folder::folder_handler;
use crate::business::workspace_tag::tag_handler;
use crate::web::middleware::auth::handle_auth;
use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, post, put};

pub fn router() -> Router {
    // 公开路由
    let public_routes = Router::new().nest("/uvwa/api", public_user_routes());

    // 受保护路由
    let protected_routes = Router::new()
        .nest("/uvwa/api", user_routes())
        .nest("/uvwa/api", workspace_routes())
        .nest("/uvwa/api", folder_routes())
        .nest("/uvwa/api", tag_routes())
        .nest("/uvwa/api", app_routes())
        .layer(middleware::from_fn(handle_auth));

    Router::new().merge(public_routes).merge(protected_routes)
}

// user routes
fn public_user_routes() -> Router {
    Router::new()
        .route("/users/sign-up", post(user_handler::sign_up))
        .route("/users/sign-in", post(user_handler::sign_in))
}

fn user_routes() -> Router {
    Router::new().route("/users", get(user_handler::list_users))
}

// Workspace routes
fn workspace_routes() -> Router {
    Router::new()
        .route("/workspaces", get(workspace_handler::list_workspaces))
        .route("/workspaces", post(workspace_handler::create_workspace))
        .route("/workspaces/{id}", put(workspace_handler::update_workspace))
        .route(
            "/workspaces/{id}",
            delete(workspace_handler::delete_workspace),
        )
        .route(
            "/workspaces/{id}/current",
            put(workspace_handler::switch_workspace),
        )
}

// Folder routes
fn folder_routes() -> Router {
    Router::new()
        .route(
            "/folders",
            get(folder_handler::get_folder_tree),
        )
        .route(
            "/folders",
            post(folder_handler::create_folder),
        )
        .route(
            "/folders/{id}",
            put(folder_handler::update_folder),
        )
        .route(
            "/folders/{id}",
            delete(folder_handler::delete_folder),
        )
        .route(
            "/folders/{id}/move",
            put(folder_handler::move_folder),
        )
}

// Tag routes
fn tag_routes() -> Router {
    Router::new()
        .route("/tags", get(tag_handler::list_tags))
        .route("/tags", post(tag_handler::create_tag))
        .route("/tags/{id}", put(tag_handler::update_tag))
        .route("/tags/{id}", delete(tag_handler::delete_tag))
}

// App routes
fn app_routes() -> Router {
    Router::new()
        .route("/apps", get(app_handler::list_apps))
        .route("/apps", post(app_handler::create_app))
        .route("/apps/{id}", put(app_handler::update_app))
        .route("/apps/{id}", delete(app_handler::delete_app))
        .route("/apps/{id}/spec", get(app_handler::get_app_spec))
        .route("/apps/{id}/spec", put(app_handler::update_app_spec))
        .route("/apps/{id}/clone", post(app_handler::clone_app))
        .route("/apps/{id}/release", post(app_handler::release_app))
        .route("/apps/{id}/tags", put(app_handler::update_app_tags))
}
