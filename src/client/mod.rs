use error::ClientError;
use watch::ClientWatcher;

mod build;
mod state;
mod watch;

pub use state::{ClientChannel, ClientChannelMessage, State as ClientState};

pub struct Client {
    watcher: Option<ClientWatcher>,
    builder: build::Builder,
    state: state::State,
}

impl Client {
    pub async fn new(watch_dir: &str) -> Self {
        let state = ClientState::new();
        let builder = build::Builder::new(state.clone()).await;
        let watcher = watch::ClientWatcher::new(watch_dir, state.clone());

        Self {
            watcher: Some(watcher),
            builder,
            state,
        }
    }

    pub fn take_watcher(&mut self) -> Result<ClientWatcher, ClientError> {
        self.watcher.take().ok_or(ClientError::WatcherAlreadyTaken)
    }

    pub fn get_state(&self) -> ClientState {
        self.state.clone()
    }
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ClientError {
        WatcherAlreadyTaken,
    }

    impl std::fmt::Display for ClientError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let message = match self {
                Self::WatcherAlreadyTaken => "The watcher can not be taken more than once",
            };

            write!(f, "{message}")
        }
    }
}
