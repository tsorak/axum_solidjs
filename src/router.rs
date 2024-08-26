use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

use crate::{api, client::ClientState};

pub fn router() -> Router<ClientState> {
    Router::new()
        .fallback_service(get_service(ServeDir::new("client/dist")))
        .nest("/api", api::api_router())
}
