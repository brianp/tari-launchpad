use tari_sdm::{
    ids::{ManagedTask, TaskId},
    network::ManagedNetwork,
};

use crate::LaunchpadConfig;

#[derive(Debug, Default)]
pub struct LocalNet {}

impl ManagedTask for LocalNet {
    fn id() -> TaskId {
        "LocalNet".into()
    }
}

impl ManagedNetwork for LocalNet {
    type Config = LaunchpadConfig;

    fn network_name(&self) -> &str {
        "network"
    }
}
