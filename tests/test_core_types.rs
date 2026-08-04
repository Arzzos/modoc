//! Unit tests for the `core::types` module.
//!
//! Tests configuration loading, default values, and error handling for YAML parsing.
//!
//! Pruebas unitarias para el módulo `core::types`.
//!
//! Prueba la carga de configuración, los valores por defecto y el manejo de errores al analizar YAML.

use modoc::core::types::{load_config, Config, ConnectionConfig, Parity};
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

/// Tests that the default configuration is as expected.
///
/// Prueba que la configuración por defecto sea la esperada.
#[test]
fn test_config_default() {
    let config = Config::default();
    match config.connection {
        ConnectionConfig::Tcp { host, port } => {
            assert_eq!(host, "127.0.0.1");
            assert_eq!(port, 502);
        }
        _ => panic!("Default should be TCP"),
    }
}

/// Tests loading a valid TCP configuration.
///
/// Prueba la carga de una configuración TCP válida.
#[test]
fn test_load_valid_tcp_config() {
    let content = r#"
connection:
  mode: tcp
  host: "192.168.1.10"
  port: 5020
"#;
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    let config = load_config(file.path()).unwrap();
    match config.connection {
        ConnectionConfig::Tcp { host, port } => {
            assert_eq!(host, "192.168.1.10");
            assert_eq!(port, 5020);
        }
        _ => panic!("Expected TCP config"),
    }
}

/// Tests loading a valid RTU configuration.
///
/// Prueba la carga de una configuración RTU válida.
#[test]
fn test_load_valid_rtu_config() {
    let content = r#"
connection:
  mode: rtu
  serial_port: "/dev/ttyUSB0"
  baud_rate: 9600
  data_bits: 8
  stop_bits: 1
  parity: odd
"#;
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    let config = load_config(file.path()).unwrap();
    match config.connection {
        ConnectionConfig::Rtu {
            serial_port,
            baud_rate,
            data_bits,
            stop_bits,
            parity,
        } => {
            assert_eq!(serial_port, "/dev/ttyUSB0");
            assert_eq!(baud_rate, 9600);
            assert_eq!(data_bits, 8);
            assert_eq!(stop_bits, 1);
            assert!(matches!(parity, Parity::Odd));
        }
        _ => panic!("Expected RTU config"),
    }
}

/// Tests loading a file that does not exist.
///
/// Prueba la carga de un archivo que no existe.
#[test]
fn test_load_missing_file() {
    let result = load_config(Path::new("/non/existent/file.yaml"));
    assert!(result.is_err());
}

/// Tests loading a malformed YAML file.
///
/// Prueba la carga de un archivo YAML malformado.
#[test]
fn test_load_malformed_yaml() {
    let content = r#"
connection:
  mode: tcp
  host: "localhost"
  port: 502
  invalid: [unclosed
"#;
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    let result = load_config(file.path());
    assert!(result.is_err());
}

/// Tests loading a YAML file with missing required fields.
///
/// Prueba la carga de un archivo YAML con campos obligatorios faltantes.
#[test]
fn test_load_missing_fields() {
    let content = r#"
connection:
  mode: tcp
  host: "localhost"
"#; // missing port
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    let result = load_config(file.path());
    assert!(result.is_err());
}
