use std::{path::PathBuf, str::FromStr, time::Duration};

use tokio::task::JoinHandle;
use watchexec::Watchexec;
use watchexec_signals::Signal;

use crate::client::{ClientChannel, ClientState};

pub struct ClientWatcher(JoinHandle<()>);

const IGNORED_PATHS: [&str; 2] = ["dist", "node_modules"];

impl ClientWatcher {
    pub fn new(watch_dir: &str, state: ClientState) -> Self {
        let watch_dir = PathBuf::from_str(watch_dir)
            .expect("Invalid watch_dir path")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let handle = tokio::spawn(async move {
            watch(state.client_channel.clone(), watch_dir)
                .await
                .expect("Watcher crashed");
        });

        Self(handle)
    }

    pub async fn wait_until_end(self) {
        let _ = self.0.await;
    }

    pub fn force_stop(&self) {
        self.0.abort()
    }
}

async fn watch(ch: ClientChannel, watch_dir: String) -> anyhow::Result<()> {
    let watch_dir2 = watch_dir.clone();
    let wx = Watchexec::new(move |mut action| {
        // if Ctrl-C is received, quit
        if action.signals().any(|sig| {
            matches!(
                sig,
                Signal::Interrupt
                    | Signal::ForceStop
                    | Signal::Quit
                    | Signal::Terminate
                    | Signal::Hangup
            )
        }) {
            action.quit_gracefully(Signal::Terminate, Duration::from_secs(10));
            println!("\n ClientWatcher exiting...");
            return action;
        }

        let files = action.paths().filter(|p| p.1.is_some());
        let allowed_paths = files
            .filter_map(|p| {
                let (p, ..) = p;
                let p = p.to_string_lossy();
                let p = p.replace(&watch_dir2, "");

                if IGNORED_PATHS.iter().any(|ignored| p.contains(ignored)) {
                    None
                } else {
                    Some(p)
                }
            })
            .collect::<Vec<String>>();

        if !allowed_paths.is_empty() {
            println!("Filechange detected, rebuilding...");
            ch.send_rebuild();
        }

        action
    })?;

    wx.config.pathset([watch_dir.as_str()]);

    let _ = wx.main().await?;

    Ok(())
}
