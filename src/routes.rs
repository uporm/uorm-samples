use crate::business::folder::folder_handler;
use axum::routing::{delete, post, put};
use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().nest("/folders/{type}", folder_routes())
}

// Folder routes
fn folder_routes() -> Router {
    Router::new()
        .route("/", get(folder_handler::get_folder_tree))
        .route("/", post(folder_handler::create_folder))
        .route("/{id}", put(folder_handler::update_folder))
        .route("/{id}", delete(folder_handler::delete_folder))
        .route("/{id}/move", put(folder_handler::move_folder))
}
