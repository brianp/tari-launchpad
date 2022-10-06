use async_trait::async_trait;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, ManagedContainer, Ports},
};

use super::GRAFANA_REGISTRY;
use crate::config::LaunchpadConfig;

#[derive(Debug, Default)]
pub struct Promtail;

impl ManagedTask for Promtail {
    fn id() -> TaskId {
        "Promtail".into()
    }
}

#[async_trait]
impl ManagedContainer for Promtail {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        GRAFANA_REGISTRY
    }

    fn image_name(&self) -> &str {
        "promtail"
    }

    async fn args(&self, args: &mut Args) {
        args.set("-config.file", "/etc/promtail/config.yml");
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_980);
    }

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        config.map(|conf| conf.with_monitoring).unwrap_or_default()
    }
}
