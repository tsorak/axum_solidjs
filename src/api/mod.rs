use axum::{response::Response, routing::get, Router};

use crate::client::ClientState;

mod ws;

pub fn api_router() -> Router<ClientState> {
    Router::new()
        .route("/", get(status::status))
        .route("/ws", get(ws::ws))
        .fallback(h_404)
}

async fn h_404() -> Response {
    lib::ext::res::text(404, "Not found")
}

pub(super) mod status {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn status() -> impl IntoResponse {
        (StatusCode::OK, "OK")
    }
}
