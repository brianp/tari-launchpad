use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Envs, ManagedContainer, Networks, Ports},
};

use super::GRAFANA_REGISTRY;
use crate::{
    config::{ConnectionSettings, LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
};

#[derive(Debug, Default)]
pub struct Grafana {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Grafana {
    fn id() -> TaskId {
        "Grafana".into()
    }
}

impl ManagedContainer for Grafana {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "grafana"
    }

    fn envs(&self, envs: &mut Envs) {
        let path = concat!(
            "/usr/share/grafana/bin:",
            "/usr/local/sbin:",
            "/usr/local/bin:",
            "/usr/sbin:",
            "/usr/bin:",
            "/sbin:",
            "/bin"
        );
        envs.set("PATH", path);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Check the `display` call is correct here?
            envs.set("DATA_FOLDER", settings.data_directory.display());
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("grafana", LocalNet::id());
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_300);
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }
}
