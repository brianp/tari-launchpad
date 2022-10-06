use async_trait::async_trait;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Ports},
};

use super::GRAFANA_REGISTRY;
use crate::config::LaunchpadConfig;

#[derive(Debug, Default)]
pub struct Loki;

impl ManagedTask for Loki {
    fn id() -> TaskId {
        "Loki".into()
    }
}

#[async_trait]
impl ManagedContainer for Loki {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "loki"
    }

    async fn args(&self, args: &mut Args) {
        args.set("-config.file", "/etc/loki/local-config.yaml");
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_310);
    }

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }
}
