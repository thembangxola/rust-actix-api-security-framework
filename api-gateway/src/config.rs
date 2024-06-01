use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_address: String,
    pub auth_secret: String,
    pub mongo_uri: String,
    pub database_name: String,
    #[serde(default)]
    pub authorization_rules: Vec<AuthorizationRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationRule {
    pub path: String,
    pub methods: Vec<String>,
    pub roles: Vec<String>,
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("configuration error")]
    Config(#[from] config::ConfigError),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("deserialization error")]
    Deserialization(#[from] serde_json::Error),
}

pub fn get_config() -> Result<Config, ConfigError> {
    let config_file = File::open("config.toml")?;
    let reader = BufReader::new(config_file);
    let mut config: Config = serde_json::from_reader(reader)?;

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| config.server_address.clone());
    let auth_secret = env::var("AUTH_SECRET").unwrap_or_else(|_| config.auth_secret.clone());
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| config.mongo_uri.clone());
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| config.database_name.clone());

    config.server_address = server_address;
    config.auth_secret = auth_secret;
    config.mongo_uri = mongo_uri;
    config.database_name = database_name;

    Ok(config)
}
