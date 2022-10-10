use std::collections::{HashMap, VecDeque};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tari_launchpad_protocol::{Incoming, LaunchpadAction, LaunchpadDelta, LaunchpadState, Outgoing};
use tari_sdm::SdmScope;
use tari_sdm_launchpad::{
    config::{LaunchpadConfig, LaunchpadProtocol},
    images,
    networks,
    volumes,
};
use tokio::{select, sync::mpsc, task::JoinHandle};

pub struct LaunchpadBus {
    // pub handle: JoinHandle<()>,
    pub incoming: mpsc::UnboundedSender<Incoming>,
    pub outgoing: mpsc::UnboundedReceiver<Outgoing>,
}

impl LaunchpadBus {
    pub fn start() -> Result<Self, Error> {
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        std::thread::spawn(move || LaunchpadWorker::create_and_run(in_rx, out_tx));
        Ok(Self {
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
    #[tokio::main]
    async fn create_and_run(
        in_rx: mpsc::UnboundedReceiver<Incoming>,
        out_tx: mpsc::UnboundedSender<Outgoing>,
    ) -> Result<(), Error> {
        let state = LaunchpadState {
            config: LaunchpadConfig::default(),
            containers: HashMap::new(),
        };
        let mut scope = SdmScope::connect("esmeralda")?;
        scope.add_network(networks::LocalNet::default())?;
        scope.add_volume(volumes::SharedVolume::default())?;
        scope.add_volume(volumes::SharedGrafanaVolume::default())?;
        scope.add_image(images::Tor::default())?;
        let worker = LaunchpadWorker {
            state,
            scope,
            in_rx,
            out_tx,
        };
        worker.entrypoint().await;
        Ok(())
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
            Incoming::Start => {
                println!("START EVENT!");
                let config = self.state.config.clone();
                self.scope.set_config(Some(config));
            },
        }
    }

    async fn process_action(&mut self, action: LaunchpadAction) {}
}
