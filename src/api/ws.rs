// use std::time::Duration;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};

use crate::client::{ClientChannelMessage, ClientState};

pub async fn ws(ws_upgrade: WebSocketUpgrade, state: State<ClientState>) -> Response {
    ws_upgrade.on_upgrade(|ws| async {
        handle_socket_connection(ws, state).await;
    })
}

async fn handle_socket_connection(mut socket: WebSocket, mut state: State<ClientState>) {
    loop {
        tokio::select! {
            // _ = tokio::time::sleep(Duration::from_secs(5)) => {
            //     let _ = socket.close().await;
            //     println!("closed the socket lol");
            //     break;
            // }
            Some(msg) = socket.recv() => {
                if let Ok(msg) = msg {
                    if let Message::Text(data) = msg {
                        dbg!(&data);
                        handle_text_message(&mut socket, &mut state, data).await;
                    }
                } else {
                    // Client disconnected
                    return;
                };

                // send the same message back
                // if socket.send(msg).await.is_err() {
                //     // Client disconnected
                //     return;
                // }
            },
            Some(m) = state.client_channel.recv() => {
                //putting "ClientChannelMessage::Refresh" in the above Some match does not work
                if matches!(m, ClientChannelMessage::Refresh) {
                    let _ = socket.send("refresh".into()).await;
                }
            }
        }
    }
}

async fn handle_text_message(
    _socket: &mut WebSocket,
    state: &mut State<ClientState>,
    data: String,
) {
    if let "rebuild" = data.as_str() {
        state.client_channel.send_rebuild();
    }
}
