use crate::error::Result;
use serde::{Deserialize, Serialize};
// runpodctl uses this config style:
// ❯ cat /Users/grw/.runpod/config.toml
// ───────┬──────────────────────────────
//        │ File: /Users/grw/.runpod/config.toml
// ───────┼──────────────────────────────
//    1   │ apikey = "APIKEY"
//    2   │ apiurl = "https://api.runpod.io/graphql"
// ───────┴──────────────────────────────
//
// We attempt to load the config from the file and then use it to create a client.

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub apikey: String,
    pub apiurl: String,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        let config_path = dirs::home_dir().unwrap().join(".runpod/config.toml");
        let config = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }
}
