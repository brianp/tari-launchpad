use tari_sdm::{
    ids::{ManagedTask, TaskId},
    volume::ManagedVolume,
};

use crate::config::{LaunchpadConfig, LaunchpadProtocol};

#[derive(Debug, Default)]
pub struct SharedVolume {}

impl ManagedTask for SharedVolume {
    fn id() -> TaskId {
        "SharedVolume".into()
    }
}

impl ManagedVolume for SharedVolume {
    type Protocol = LaunchpadProtocol;

    fn volume_name(&self) -> &str {
        "volume"
    }

    fn reconfigure(&mut self, _config: Option<&LaunchpadConfig>) -> bool {
        true
    }
}
