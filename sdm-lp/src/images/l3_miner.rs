use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Mounts, Networks, Volumes},
};
use tari_sdm::image::Envs;

use super::{TariBaseNode, TariWallet, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    images::VAR_TARI_PATH,
    networks::LocalNet,
    volumes::SharedVolume,
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
        vec![TariBaseNode::id(), TariWallet::id(), LocalNet::id(), SharedVolume::id()]
    }
}

impl ManagedContainer for TariSha3Miner {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_sha3_miner"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor(envs);
            envs.set("WAIT_FOR_TOR", 6);
            envs.set("TARI_MINER__NUM_MINING_THREADS", 8); // TODO: Get config num
            envs.set("TARI_MINER__MINE_ON_TIP_ONLY", 1);
            envs.set(&format!("TARI_BASE_NODE__{}__GRPC_BASE_NODE_GRPC_ADDRESS", settings.tari_network.upper_case()), "/dns4/base_node/tcp/18142");
            envs.set("TARI_WALLET__GRPC_ADDRESS", "/dns4/wallet/tcp/18143");
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "sha3_miner");
        envs.set("APP_EXEC", "tari_miner");
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
