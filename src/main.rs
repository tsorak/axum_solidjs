use tokio::net::TcpListener;

mod api;
mod router;

mod client;
use client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tcp_listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind on port 3000");

    println!("Listening on http://localhost:3000");

    let mut client = Client::new("./client").await;
    let client_watcher = client.take_watcher()?;

    client.ensure_node_modules().await?;
    client.build_client().await;

    tokio::select! {
        _ = async move { axum::serve(tcp_listener, router::router().with_state(client.get_state())).await } => {
            client_watcher.force_stop();
            client_watcher.wait_until_end().await;
        },
        _ = tokio::signal::ctrl_c() => {
            //ClientWatcher handles signals on its own, just wait for it to shutdown
            client_watcher.wait_until_end().await;
        }
    }

    Ok(())
}
