use tari_sdm::{
    ids::{ManagedTask, TaskId},
    network::ManagedNetwork,
};

use crate::config::{LaunchpadConfig, LaunchpadProtocol};

#[derive(Debug, Default)]
pub struct LocalNet {}

impl ManagedTask for LocalNet {
    fn id() -> TaskId {
        "LocalNet".into()
    }
}

impl ManagedNetwork for LocalNet {
    type Protocol = LaunchpadProtocol;

    fn network_name(&self) -> &str {
        "network"
    }
}
