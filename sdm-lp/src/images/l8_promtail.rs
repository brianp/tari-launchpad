use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Mounts, Networks, Ports},
};

use super::{GRAFANA_PATH, GRAFANA_REGISTRY};
use crate::{
    config::{LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct Promtail;

impl ManagedTask for Promtail {
    fn id() -> TaskId {
        "Promtail".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id()]
    }
}

impl ManagedContainer for Promtail {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "promtail"
    }

    fn args(&self, args: &mut Args) {
        args.set("-config.file", "/etc/promtail/config.yml");
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("promtail", LocalNet::id());
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_980);
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }

    fn mounts(&self, mounts: &mut Mounts) {
        mounts.add_volume(SharedVolume::id(), GRAFANA_PATH);
    }
}
