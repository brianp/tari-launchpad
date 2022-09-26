use tari_sdm::{
    ids::{ManagedTask, TaskId},
    volume::ManagedVolume,
};

use crate::LaunchpadConfig;

#[derive(Debug, Default)]
pub struct SharedVolume {}

impl ManagedTask for SharedVolume {
    fn id() -> TaskId {
        "SharedVolume".into()
    }
}

impl ManagedVolume for SharedVolume {
    type Config = LaunchpadConfig;

    fn volume_name(&self) -> &str {
        "volume"
    }

    fn reconfigure(&mut self, _config: Option<&Self::Config>) -> bool {
        true
    }
}
