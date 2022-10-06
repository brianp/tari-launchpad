use anyhow::Error;
use tokio::fs::create_dir_all;

const CONFIG_TOML: ConfigFile = ConfigFile::new("config.toml", "../../backend/assets/config.toml");
const DEFAULTS_INI: ConfigFile = ConfigFile::new("defaults.ini", "../../backend/assets/defaults.ini");
const LOGS4RS_YML: ConfigFile = ConfigFile::new("log4rs.yml", "../../backend/assets/log4rs.yml");
const LOKI_YML: ConfigFile = ConfigFile::new("loki_config.yml", "../../backend/assets/loki_config.yml");
const PROMTAIL_YML: ConfigFile = ConfigFile::new("promtail.config.yml", "../backend/assets/promtail.config.yml");
const PROVISION_YML: ConfigFile =
    ConfigFile::new("sources_provision.yml", "../../backend/assets/sources_provision.yml");

struct ConfigFile {
    filename: &'static str,
    data: &'static str,
}

impl ConfigFile {
    const fn new(filename: &'static str, data: &'static str) -> Self {
        Self { filename, data }
    }
}

pub struct Configurator {
    working_dir: String,
}

impl Configurator {
    async fn create_folders(&mut self) -> Result<(), Error> {
        create_dir_all(&self.working_dir).await?;
        Ok(())
    }
}
