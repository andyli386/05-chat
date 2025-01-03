use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yml::from_reader(reader),
            (_, Ok(reader), _) => serde_yml::from_reader(reader),
            (_, _, Ok(path)) => serde_yml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}
