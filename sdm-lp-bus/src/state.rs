use std::collections::{HashMap, VecDeque};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tari_sdm::SdmScope;
use tari_sdm_launchpad::config::{LaunchpadConfig, LaunchpadProtocol};
use tokio::{select, sync::mpsc};

pub struct ContainerRecord {
    pub logs: VecDeque<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchpadState {
    pub config: LaunchpadConfig,
    pub containers: HashMap<String, String>,
}

impl LaunchpadState {
    pub fn apply(&mut self, delta: LaunchpadDelta) {
        use LaunchpadDelta::*;
        match delta {
            UpdateConfig(config) => {
                self.config = config;
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LaunchpadDelta {
    UpdateConfig(LaunchpadConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Outgoing {
    StateIsReady(LaunchpadState),
    Delta(LaunchpadDelta),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LaunchpadAction {
    Connect,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Incoming {
    Action(LaunchpadAction),
}

pub struct LaunchpadWorker {
    state: LaunchpadState,
    scope: SdmScope<LaunchpadProtocol>,
    in_rx: mpsc::UnboundedReceiver<Incoming>,
    out_tx: mpsc::UnboundedSender<Outgoing>,
}

impl LaunchpadWorker {
    async fn init() -> Result<Self, Error> {
        let state = LaunchpadState {
            config: LaunchpadConfig::default(),
            containers: HashMap::new(),
        };
        let mut scope = SdmScope::connect("esmeralda")?;
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        Ok(Self {
            state,
            scope,
            in_rx,
            out_tx,
        })
    }

    async fn entrypoint(mut self) {
        loop {
            select! {
                action = self.in_rx.recv() => {
                    if let Some(action) = action {
                        self.process_incoming(action).await
                    }
                }
            }
        }
    }

    async fn process_incoming(&mut self, incoming: Incoming) {
        match incoming {
            Incoming::Action(action) => self.process_action(action).await,
        }
    }

    async fn process_action(&mut self, action: LaunchpadAction) {}
}
