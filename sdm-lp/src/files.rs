use std::path::PathBuf;

use anyhow::Error;
use tokio::fs;

macro_rules! embed_file {
    ($f:literal) => {
        ConfigFile::new($f, concat!("../../backend/assets/", $f))
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

    async fn create_dir(&mut self, folder: &PathBuf) -> Result<(), Error> {
        if !folder.exists() {
            fs::create_dir_all(folder).await?;
        }
        Ok(())
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
        let mut path = self.base_dir.clone();
        self.create_dir(&path).await?;
        // config folder
        path.push("config");
        self.create_dir(&path).await?;
        // config files
        self.store_file(&path, &CONFIG_TOML).await?;
        self.store_file(&path, &DEFAULTS_INI).await?;
        self.store_file(&path, &LOGS4RS_YML).await?;
        self.store_file(&path, &LOKI_YML).await?;
        self.store_file(&path, &PROMTAIL_YML).await?;
        self.store_file(&path, &PROVISION_YML).await?;
        Ok(())
    }

    async fn remove_configuration(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
