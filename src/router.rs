use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::{api, client::ClientState};

pub fn router() -> Router<ClientState> {
    index_router().nest("/api", api::api_router())
}

fn index_router() -> Router<ClientState> {
    Router::new().fallback_service(
        ServeDir::new("./client/dist/").fallback(ServeFile::new("./client/dist/index.html")),
    )
}
