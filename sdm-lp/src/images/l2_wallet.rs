use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{Args, Envs, ManagedContainer, Mounts, Networks, Ports, Volumes},
};
use tari_utilities::hex::Hex;

use super::{TariBaseNode, DEFAULT_REGISTRY, GENERAL_VOLUME};
use crate::{
    config::{BaseNodeIdentity, ConnectionSettings, LaunchpadConfig, LaunchpadInnerEvent, LaunchpadProtocol},
    images::{BLOCKCHAIN_PATH, VAR_TARI_PATH},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct TariWallet {
    settings: Option<ConnectionSettings>,
    identity: Option<BaseNodeIdentity>,
}

impl ManagedTask for TariWallet {
    fn id() -> TaskId {
        "Wallet".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id(), TariBaseNode::id()]
    }
}

impl ManagedContainer for TariWallet {
    type Protocol = LaunchpadProtocol;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tari_wallet"
    }

    fn reconfigure(&mut self, config: Option<&LaunchpadConfig>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn on_event(&mut self, event: LaunchpadInnerEvent) {
        match event {
            LaunchpadInnerEvent::IdentityReady(identity) => {
                self.identity = Some(identity);
            },
        }
    }

    fn ports(&self, ports: &mut Ports) {
        ports.add(18_143);
        ports.add(18_188);
    }

    fn args(&self, args: &mut Args) {
        args.set("--log-config", "/var/tari/config/log4rs.yml");
        args.set("--seed-words-file", "/var/tari/config/seed_words.txt");
        args.flag("--enable-grpc");
        args.flag("-n");

        if let Some(identity) = self.identity.as_ref() {
            let value = format!(
                "wallet.custom_base_node={}::{}",
                identity.public_key.to_hex(),
                identity.public_address
            );
            args.set_pair("-p", value);
        }
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.add_common(envs);
            settings.add_tor(envs);
            envs.set("WAIT_FOR_TOR", 0);
            envs.set(
                "TARI_BASE_NODE__DATA_DIR",
                format!("/blockchain/{}", settings.tari_network.lower_case()),
            );
            envs.set("TARI_WALLET_PASSWORD", &settings.wallet_password); // HERE
            envs.set(
                "TARI_WALLET__P2P__TRANSPORT__TOR__CONTROL_AUTH",
                format!("password={}", &settings.tor_password),
            );
        }
        envs.set("SHELL", "/bin/bash");
        envs.set("TERM", "linux");
        envs.set("APP_NAME", "wallet");
        envs.set("APP_EXEC", "tari_console_wallet");
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add("wallet", LocalNet::id());
    }

    fn volumes(&self, volumes: &mut Volumes) {
        volumes.add(GENERAL_VOLUME);
    }

    fn mounts(&self, mounts: &mut Mounts) {
        if let Some(settings) = self.settings.as_ref() {
            // TODO: Avoid using display here
            mounts.bind_path(settings.data_directory.display(), VAR_TARI_PATH);
            mounts.add_volume(SharedVolume::id(), BLOCKCHAIN_PATH);
        }
    }
}
