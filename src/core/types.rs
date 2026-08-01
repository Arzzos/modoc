use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum ConnectionConfig {
    Tcp {
        host: String,
        port: u16,
    },
    Rtu {
        serial_port: String,
        baud_rate: u32,
        data_bits: u8,
        stop_bits: u8,
        parity: Parity,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Parity {
    None,
    Odd,
    Even,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::Tcp {
                host: "127.0.0.1".to_string(),
                port: 502,
            },
        }
    }
}

pub fn load_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
