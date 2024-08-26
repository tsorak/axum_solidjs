use axum::{routing::get, Router};

use crate::client::ClientState;

mod ws;

pub fn api_router() -> Router<ClientState> {
    Router::new()
        .route("/", get(status::status))
        .route("/ws", get(ws::ws))
}

pub(super) mod status {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn status() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
}
