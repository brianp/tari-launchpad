use std::collections::{HashMap, VecDeque};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tari_launchpad_protocol::{Incoming, LaunchpadAction, LaunchpadDelta, LaunchpadState, Outgoing};
use tari_sdm::SdmScope;
use tari_sdm_launchpad::config::{LaunchpadConfig, LaunchpadProtocol};
use tokio::{select, sync::mpsc, task::JoinHandle};

pub struct LaunchpadBus {
    pub handle: JoinHandle<()>,
    pub incoming: mpsc::UnboundedSender<Incoming>,
    pub outgoing: mpsc::UnboundedReceiver<Outgoing>,
}

impl LaunchpadBus {
    pub fn start() -> Result<Self, Error> {
        let state = LaunchpadState {
            config: LaunchpadConfig::default(),
            containers: HashMap::new(),
        };

        let mut scope = SdmScope::connect("esmeralda")?;
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        let worker = LaunchpadWorker {
            state,
            scope,
            in_rx,
            out_tx,
        };
        let handle = tokio::spawn(worker.entrypoint());
        Ok(Self {
            handle,
            incoming: in_tx,
            outgoing: out_rx,
        })
    }
}

pub struct LaunchpadWorker {
    state: LaunchpadState,
    scope: SdmScope<LaunchpadProtocol>,
    in_rx: mpsc::UnboundedReceiver<Incoming>,
    out_tx: mpsc::UnboundedSender<Outgoing>,
}

impl LaunchpadWorker {
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
