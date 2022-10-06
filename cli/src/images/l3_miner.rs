use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{ManagedContainer, Volumes},
};

use super::{TariBaseNode, TariWallet, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::config::LaunchpadProtocol;

#[derive(Debug)]
pub struct TariSha3Miner;

impl ManagedTask for TariSha3Miner {
    fn id() -> TaskId {
        "Sha3Miner".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![TariBaseNode::id(), TariWallet::id()]
    }
}

impl ManagedContainer for TariSha3Miner {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_miner"
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }
}
