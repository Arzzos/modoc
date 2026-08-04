//! Data types and configuration structures for Modoc.
//!
//! Defines the configuration file format and connection parameters for
//! both TCP and RTU Modbus communication modes.

//! Tipos de datos y estructuras de configuración para Modoc.
//!
//! Define el formato del archivo de configuración y los parámetros de conexión
//! para los modos de comunicación Modbus TCP y RTU.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Top-level configuration structure for the application.
///
/// Estructura de configuración de nivel superior para la aplicación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
}

/// Connection configuration for Modbus communication.
///
/// Supports both TCP and RTU (serial) modes with their respective parameters.
///
/// Configuración de conexión para la comunicación Modbus.
///
/// Soporta ambos modos TCP y RTU (serie) con sus respectivos parámetros.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum ConnectionConfig {
    /// TCP connection over Ethernet
    ///
    /// Conexión TCP a través de Ethernet
    Tcp { host: String, port: u16 },
    /// RTU connection over serial port
    ///
    /// Conexión RTU a través de puerto serie
    Rtu {
        serial_port: String,
        baud_rate: u32,
        data_bits: u8,
        stop_bits: u8,
        parity: Parity,
    },
}

/// Serial parity configuration for RTU mode.
///
/// Configuración de paridad serie para el modo RTU.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Parity {
    None,
    Odd,
    Even,
}

/// Default configuration for the application.
///
/// Provides a sensible default for TCP connection to localhost on port 502.
///
/// Configuración predeterminada para la aplicación.
///
/// Proporciona un valor predeterminado sensato para conexión TCP a localhost en el puerto 502.
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

/// Loads and parses a YAML configuration file.
///
/// # Arguments
/// * `path` - Path to the YAML configuration file
///
/// # Returns
/// The parsed `Config` structure, or an error if the file cannot be read or parsed.
///
/// Carga y analiza un archivo de configuración YAML.
///
/// # Argumentos
/// * `path` - Ruta al archivo de configuración YAML
///
/// # Retorna
/// La estructura `Config` analizada, o un error si el archivo no puede ser leído o analizado.
pub fn load_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
