use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{ManagedContainer, Ports, Volumes},
};

use super::{TariBaseNode, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::config::LaunchpadConfig;

#[derive(Debug)]
pub struct TariWallet;

impl ManagedTask for TariWallet {
    fn id() -> TaskId {
        "Wallet".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![TariBaseNode::id()]
    }
}

impl ManagedContainer for TariWallet {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_wallet"
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_143);
        ports.add(18_188);
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }
}
