use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Networks, Ports},
};

use super::GRAFANA_REGISTRY;
use crate::{
    config::{LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct Loki;

impl ManagedTask for Loki {
    fn id() -> TaskId {
        "Loki".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id()]
    }
}

impl ManagedContainer for Loki {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "loki"
    }

    fn args(&self, args: &mut Args) {
        args.set("-config.file", "/etc/loki/local-config.yaml");
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("loki", LocalNet::id());
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_310);
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }
}
