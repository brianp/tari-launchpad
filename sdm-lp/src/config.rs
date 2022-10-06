// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use std::path::PathBuf;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tari_base_node_grpc_client::grpc::NodeIdentity;
use tari_common_types::{emoji::EmojiId, types::PublicKey};
use tari_sdm::{config::ManagedProtocol, image::Envs};
use tari_utilities::{ByteArray, Hidden};
use tari_wallet_grpc_client::grpc::GetIdentityResponse;
use thiserror::Error;

pub const DEFAULT_MONEROD_URL: &str = "http://stagenet.xmr-tw.org:38081,\
http://stagenet.community.xmr.to:38081,\
http://monero-stagenet.exan.tech:38081,\
http://xmr-lux.boldsuck.org:38081,\
http://singapore.node.xmr.pm:38081";

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BaseNodeConfig {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    /// The password to de/en-crypt the wallet database
    #[serde(skip_serializing)]
    pub password: Hidden<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct XmRigConfig {
    /// The address that will accept Monero mining rewards
    pub monero_mining_address: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Sha3MinerConfig {
    /// The number of threads to employ for SHA3 mining
    pub num_mining_threads: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MmProxyConfig {
    /// A URL specifying the Monero daemon to connect to
    pub monerod_url: String,
    /// If required, the monero username for the monero daemon
    pub monero_username: String,
    /// If required, the password needed to access the monero deamon
    #[serde(skip_serializing)]
    pub monero_password: Hidden<String>,
    /// If true, provide the monero username and password to the daemon. Otherwise those strings are ignored.
    pub monero_use_auth: bool,
}

impl Default for MmProxyConfig {
    fn default() -> Self {
        MmProxyConfig {
            monerod_url: DEFAULT_MONEROD_URL.to_string(),
            monero_username: "".to_string(),
            monero_password: Hidden::default(),
            monero_use_auth: false,
        }
    }
}

impl MmProxyConfig {
    pub fn monero_use_auth(&self) -> usize {
        if self.monero_use_auth {
            1
        } else {
            0
        }
    }
}

/// Tari Launchpad configuration struct. This will generally
/// be populated from some front-end or persistent storage
/// file and is used to generate the environment variables
/// needed to configure and run the various docker containers.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct LaunchpadConfig {
    /// The directory to use for config, id files and logs
    pub data_directory: PathBuf,
    /// The Tari network to use. Default = esmeralda
    pub tari_network: TariNetwork,
    /// The tor control password to share among containers.
    pub tor_control_password: Hidden<String>,
    /// Whether to spin up a base node or not, with
    /// the given configuration. Usually you want this.
    pub base_node: Option<BaseNodeConfig>,
    /// Whether to spin up a console wallet daemon,
    /// with the given configuration. Optional.
    pub wallet: Option<WalletConfig>,
    /// Whether to spin up a SHA3 miner or not,
    /// with the given configuration. If you want
    /// to mine Tari natively, include this.
    pub sha3_miner: Option<Sha3MinerConfig>,
    /// Whether to spin up a merge-mine proxy or not,
    /// with the given configuration. If included,
    /// you must also include
    /// xmrig
    pub mm_proxy: Option<MmProxyConfig>,
    /// Whether to spin up a Monero miner or not,
    /// with the given configuration. If included
    /// you should also include
    /// mm_proxy
    pub xmrig: Option<XmRigConfig>,
    /// The Docker registry to use to download images.
    /// By default we use quay.io
    pub registry: Option<String>,
    /// The docker tag to use. By default, we use 'latest'
    pub tag: Option<String>,

    pub with_monitoring: bool,
}

#[derive(Debug)]
pub struct LaunchpadProtocol;

impl ManagedProtocol for LaunchpadProtocol {
    type Config = LaunchpadConfig;
    type Inner = LaunchpadInnerEvent;
}

#[derive(Debug, Clone)]
pub enum LaunchpadInnerEvent {
    IdentityReady(BaseNodeIdentity),
    WalletIdentityReady(WalletIdentity),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseNodeIdentity {
    pub public_key: Vec<u8>,
    pub public_address: String,
    node_id: Vec<u8>,
    emoji_id: String,
}

impl TryFrom<NodeIdentity> for BaseNodeIdentity {
    type Error = Error;

    fn try_from(value: NodeIdentity) -> Result<Self, Self::Error> {
        let public_key = PublicKey::from_bytes(&value.public_key)?;
        // TODO: Implement `Serialize` for `EmojiId`
        let emoji_id = EmojiId::from_public_key(&public_key).to_string();
        Ok(BaseNodeIdentity {
            public_key: value.public_key,
            public_address: value.public_address,
            node_id: value.node_id,
            emoji_id,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletIdentity {
    public_key: Vec<u8>,
    public_address: String,
    node_id: Vec<u8>,
    emoji_id: String,
}

impl TryFrom<GetIdentityResponse> for WalletIdentity {
    type Error = Error;

    fn try_from(value: GetIdentityResponse) -> Result<Self, Self::Error> {
        let public_key = PublicKey::from_bytes(&value.public_key)?;
        let emoji_id = EmojiId::from_public_key(&public_key).to_string();
        Ok(WalletIdentity {
            public_key: value.public_key,
            public_address: value.public_address,
            node_id: value.node_id,
            emoji_id,
        })
    }
}

#[derive(Debug)]
pub struct ConnectionSettings {
    pub tor_password: Hidden<String>,
    pub tari_network: TariNetwork,
    pub data_directory: PathBuf,
    pub wallet_password: Hidden<String>,
}

impl<'a> From<&'a LaunchpadConfig> for ConnectionSettings {
    fn from(config: &'a LaunchpadConfig) -> Self {
        ConnectionSettings {
            tor_password: config.tor_control_password.clone(),
            tari_network: config.tari_network,
            data_directory: config.data_directory.clone(),
            wallet_password: config
                .wallet
                .clone()
                .unwrap_or_else(|| WalletConfig::default())
                .password,
        }
    }
}

impl ConnectionSettings {
    pub fn add_tor(&self, envs: &mut Envs) {
        let value = format!("password={}", self.tor_password.reveal());
        envs.set("TARI_BASE_NODE__P2P__TRANSPORT__TOR__CONTROL_AUTH", value);
    }

    pub fn add_common(&self, envs: &mut Envs) {
        envs.set("TARI_NETWORK", self.tari_network.lower_case());
        envs.set("DATA_FOLDER", self.data_directory.to_str().unwrap_or(""));
        envs.set("TARI_LOG_CONFIGURATION", "/var/tari/config/log4rs.yml");
        let path = concat!(
            "/usr/local/sbin:",
            "/usr/local/bin:",
            "/usr/sbin:",
            "/usr/bin:",
            "/sbin:",
            "/bin",
        );
        envs.set("PATH", path);
    }
}

#[derive(Debug, Error)]
#[error("Unsupported network: {0}")]
pub struct UnsupportedNetwork(String);

/// Supported networks for the launchpad
#[derive(Serialize, Debug, Deserialize, Clone, Copy)]
pub enum TariNetwork {
    Dibbler,
    Esmeralda,
    Igor,
    Mainnet,
}

impl TariNetwork {
    pub fn lower_case(self) -> &'static str {
        match self {
            Self::Dibbler => "dibbler",
            Self::Esmeralda => "esmeralda",
            Self::Igor => "igor",
            Self::Mainnet => "mainnet",
        }
    }

    pub fn upper_case(self) -> &'static str {
        match self {
            Self::Dibbler => "DIBBLER",
            Self::Esmeralda => "ESMERALDA",
            Self::Igor => "IGOR",
            Self::Mainnet => "MAINNET",
        }
    }
}

/// Default network is Esme. This will change after mainnet launch
impl Default for TariNetwork {
    fn default() -> Self {
        Self::Esmeralda
    }
}

impl TryFrom<&str> for TariNetwork {
    type Error = UnsupportedNetwork;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dibbler" => Ok(TariNetwork::Dibbler),
            "esmeralda" => Ok(TariNetwork::Esmeralda),
            "igor" => Ok(TariNetwork::Igor),
            "mainnet" => Ok(TariNetwork::Mainnet),
            other => Err(UnsupportedNetwork(other.to_owned())),
        }
    }
}
