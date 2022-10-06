use std::path::PathBuf;

use anyhow::Error;
use tokio::fs;

use crate::config::LaunchpadConfig;

macro_rules! embed_file {
    ($f:literal) => {
        ConfigFile::new($f, include_str!(concat!("../../backend/assets/", $f)))
    };
}

const CONFIG_TOML: ConfigFile = embed_file!("config.toml");
const DEFAULTS_INI: ConfigFile = embed_file!("defaults.ini");
const LOGS4RS_YML: ConfigFile = embed_file!("log4rs.yml");
const LOKI_YML: ConfigFile = embed_file!("loki_config.yml");
const PROMTAIL_YML: ConfigFile = embed_file!("promtail.config.yml");
const PROVISION_YML: ConfigFile = embed_file!("sources_provision.yml");

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
    base_dir: PathBuf,
}

impl Configurator {
    pub fn init() -> Result<Self, Error> {
        let cache_dir = dirs_next::cache_dir().ok_or_else(|| Error::msg("No cache dir"))?;
        let mut data_directory = PathBuf::from(cache_dir);
        data_directory.push("tari-launchpad");
        Ok(Self {
            base_dir: data_directory,
        })
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_dir
    }

    // pub async fn read_config(&self) -> Result<LaunchpadConfig, Error> {
    // let mut path = self.base_dir.clone();
    // path.push("config");
    // path.push("config.toml");
    // let data = fs::read_to_string(&path).await?;
    // let config = toml::from_str(&data)?;
    // Ok(config)
    // }

    async fn create_dir(&mut self, folder: &PathBuf) -> Result<(), Error> {
        if !folder.exists() {
            fs::create_dir_all(&folder).await?;
        }
        Ok(())
    }

    async fn create_sub_dir(&mut self, folder: &PathBuf, sub_path: &str) -> Result<PathBuf, Error> {
        let mut path = folder.clone();
        path.push(sub_path);
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }
        Ok(path)
    }

    async fn store_file(&mut self, folder: &PathBuf, file: &ConfigFile) -> Result<(), Error> {
        let mut path = folder.clone();
        path.push(file.filename);
        if !path.exists() {
            fs::write(path, file.data).await?;
        }
        Ok(())
    }

    pub async fn repair_configuration(&mut self) -> Result<(), Error> {
        // base path
        let base_dir = self.base_dir.clone();
        self.create_dir(&base_dir).await?;
        let config_dir = self.create_sub_dir(&base_dir, "config").await?;
        // config files
        self.store_file(&config_dir, &CONFIG_TOML).await?;
        self.store_file(&config_dir, &DEFAULTS_INI).await?;
        self.store_file(&config_dir, &LOGS4RS_YML).await?;
        self.store_file(&config_dir, &LOKI_YML).await?;
        self.store_file(&config_dir, &PROMTAIL_YML).await?;
        self.store_file(&config_dir, &PROVISION_YML).await?;

        // TODO: Use `enum` here...
        // images
        self.create_sub_dir(&base_dir, "tor").await?;
        self.create_sub_dir(&base_dir, "base_node").await?;
        self.create_sub_dir(&base_dir, "wallet").await?;
        self.create_sub_dir(&base_dir, "xmrig").await?;
        self.create_sub_dir(&base_dir, "sha3_miner").await?;
        self.create_sub_dir(&base_dir, "mm_proxy").await?;
        self.create_sub_dir(&base_dir, "monerod").await?;
        self.create_sub_dir(&base_dir, "grafana").await?;
        Ok(())
    }

    async fn remove_configuration(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
