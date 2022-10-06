use anyhow::Error;
use async_trait::async_trait;
use tari_base_node_grpc_client::{grpc, BaseNodeGrpcClient};
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{
        checker::{CheckerContext, CheckerEvent, ContainerChecker},
        Args,
        Envs,
        ManagedContainer,
        Mounts,
        Networks,
        Ports,
        Volumes,
    },
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

#[async_trait]
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

    fn checker(&mut self) -> Box<dyn ContainerChecker> {
        Box::new(Checker::new())
    }

    async fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.flag("-n");
        args.set("--watch", "status");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor(envs);
            envs.set("WAIT_FOR_TOR", 10);
            envs.set(
                "TARI_BASE_NODE__DATA_DIR",
                format!("/blockchain/{}", settings.tari_network.lower_case()),
            );
        }
        envs.set("APP_NAME", "base_node");
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_142);
        ports.add(18_189);
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("base_node", LocalNet::id());
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

pub struct Checker {}

impl Checker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ContainerChecker for Checker {
    async fn on_interval(&mut self, ctx: &mut CheckerContext) -> Result<(), Error> {
        // TODO: Keep the client
        let mut client = BaseNodeGrpcClient::connect("http://127.0.0.1:18142").await?;
        let progress = client.get_sync_progress(grpc::Empty {}).await?.into_inner();
        let current = progress.local_height;
        let total = progress.tip_height;
        let pct = current as f32 / total as f32 * 100.0;
        ctx.send(CheckerEvent::Progress(pct as u8)).ok();
        if current == total && total != 0 {
            ctx.send(CheckerEvent::Ready).ok();
        }
        Ok(())
    }
}
