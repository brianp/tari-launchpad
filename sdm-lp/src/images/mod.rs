mod l1_tor;
mod l2_base_node;
mod l2_wallet;
mod l3_miner;
mod l8_grafana;
mod l8_loki;
mod l8_promtail;
mod sync_progress;

pub use l1_tor::Tor;
pub use l2_base_node::TariBaseNode;
pub use l2_wallet::TariWallet;
pub use l3_miner::TariSha3Miner;
pub use l8_grafana::Grafana;
pub use l8_loki::Loki;
pub use l8_promtail::Promtail;

static DEFAULT_REGISTRY: &str = "quay.io/tarilabs";
static GRAFANA_REGISTRY: &str = "grafana";

static GENERAL_VOLUME: &str = "/var/tari";
static BLOCKCHAIN_VOLUME: &str = "/blockchain";
static GRAFANA_VOLUME: &str = "/grafana";

static VAR_TARI_PATH: &str = "/var/tari";
static BLOCKCHAIN_PATH: &str = "/blockchain";
static GRAFANA_DEFAULTS_PATH: &str = "/usr/share/grafana/conf/defaults.ini";
static GRAFANA_PROVISION_PATH: &str = "/etc/grafana/provisioning/datasources/all.yml";
