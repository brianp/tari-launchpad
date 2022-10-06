use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Mounts, Networks, Volumes},
};

use super::{TariBaseNode, TariWallet, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::VAR_TARI_PATH,
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct TariSha3Miner {
    settings: Option<ConnectionSettings>,
}

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

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("tari_sha3_miner", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
        }
    }
}
