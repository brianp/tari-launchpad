use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Mounts, Networks, Ports, Volumes},
};

use super::{Tor, BLOCKCHAIN_PATH, BLOCKCHAIN_VOLUME, DEFAULT_REGISTRY, GENERAL_VOLUME, VAR_TARI_PATH};
use crate::{
    config::{ConnectionSettings, LaunchpadConfig},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariBaseNode {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for TariBaseNode {
    fn id() -> TaskId {
        "Base Node".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), Tor::id()]
    }
}

impl ManagedContainer for TariBaseNode {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_base_node"
    }

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.flag("-n");
        args.set("--watch", "status");
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_142);
        ports.add(18_189);
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add(LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
        volumes.add(BLOCKCHAIN_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.add_volume(SharedVolume::id(), BLOCKCHAIN_PATH);
        }
    }
}
