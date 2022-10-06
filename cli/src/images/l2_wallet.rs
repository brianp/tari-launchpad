use async_trait::async_trait;
use tari_base_node_grpc_client::{BaseNodeGrpcClient, grpc};
use tari_common_types::types::PublicKey;
use tari_utilities::{byte_array::ByteArray, hex::Hex};
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{ManagedContainer, Ports, Volumes},
};
use tari_sdm::image::{Args, Envs};

use super::{TariBaseNode, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::{
    config::{ConnectionSettings, LaunchpadConfig},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariWallet {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for TariWallet {
    fn id() -> TaskId {
        "Wallet".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), TariBaseNode::id()]
    }
}

#[async_trait]
impl ManagedContainer for TariWallet {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_wallet"
    }

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_143);
        ports.add(18_188);
    }

    async fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.set("--seed-words-file", "/var/tari/config/seed_words.txt");
        args.flag("--enable-grpc");
        args.flag("-n");

        if let Ok(mut client) = BaseNodeGrpcClient::connect("http://127.0.0.1:18142").await {
            if let Ok(rep) = client.identify(grpc::Empty {}).await.map_err(|e| e.to_string()) {
                let identity = rep.into_inner();
                let pub_key = PublicKey::from_bytes(&identity.public_key).expect("Couldn't convert the pubkey");

                args.set("-p", format!("wallet.custom_base_node={}::{}", pub_key.to_hex(), identity.public_address));
            }
        }

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
            envs.set("TARI_WALLET_PASSWORD", &settings.wallet_password); // HERE
            envs.set("TARI_WALLET__P2P__TRANSPORT__TOR__CONTROL_AUTH", format!("password={}", &settings.tor_password));
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "wallet");
        envs.set("APP_EXEC", "tari_console_wallet");
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }
}
