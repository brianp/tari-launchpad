use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Mounts, Networks, Ports},
};
use tari_sdm::image::{Envs, Volumes};

use super::{GRAFANA_PATH, GRAFANA_REGISTRY};
use crate::{
    config::{LaunchpadConfig, LaunchpadProtocol},
    networks::LocalNet,
    volumes::SharedVolume,
};
use crate::config::ConnectionSettings;
use crate::images::{GENERAL_VOLUME, Grafana, GRAFANA_VOLUME, PROMTAIL_CONFIG_PATH, VAR_TARI_PATH};
use crate::volumes::SharedGrafanaVolume;

#[derive(Debug, Default)]
pub struct Promtail {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Promtail {
    fn id() -> TaskId {
        "Promtail".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), SharedGrafanaVolume::id(), Grafana::id()]
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
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        mounts.add_volume(SharedGrafanaVolume::id(), GRAFANA_VOLUME);
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.bind_path(
                settings.data_directory.join("config").join("promtail.config.yml").display(),
                PROMTAIL_CONFIG_PATH,
            );
        }
    }
}
