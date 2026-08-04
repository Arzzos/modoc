//! Integration tests for the modoc CLI.
//!
//! These tests spin up a mock slave and verify read/write operations for all register types,
//! as well as error handling for invalid configurations and out-of-range addresses.
//!
//! Pruebas de integración para la CLI modoc.
//!
//! Estas pruebas inician un esclavo simulado y verifican las operaciones de lectura/escritura
//! para todos los tipos de registros, así como el manejo de errores para configuraciones
//! inválidas y direcciones fuera de rango.

use assert_cmd::Command;
use std::fs;
use std::net::TcpListener;
use std::process::{Child, Command as StdCommand, Stdio};
use tempfile::NamedTempFile;

/// Finds an available port by binding to port 0.
///
/// Encuentra un puerto disponible vinculándose al puerto 0.
fn find_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

/// Starts a `modoc serve` process in the background on a free port.
/// Returns the port and the child process handle.
///
/// Inicia un proceso `modoc serve` en segundo plano en un puerto libre.
/// Retorna el puerto y el manejador del proceso hijo.
fn start_slave_server() -> (u16, Child) {
    let port = find_free_port();
    let port_str = port.to_string();
    let bin = assert_cmd::cargo::cargo_bin("modoc");
    let child = StdCommand::new(bin)
        .arg("serve")
        .arg("-e")
        .arg(&port_str)
        .arg("-m")
        .arg("tcp")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start modoc serve");
    // Give the server a moment to start
    std::thread::sleep(std::time::Duration::from_millis(150));
    (port, child)
}

/// Creates a temporary configuration file with the given port.
///
/// Crea un archivo de configuración temporal con el puerto dado.
fn create_temp_config(port: u16) -> NamedTempFile {
    let config = format!(
        r#"
connection:
  mode: tcp
  host: "127.0.0.1"
  port: {}
"#,
        port
    );
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), config).unwrap();
    file
}

/// Creates a temporary configuration file with a valid but unused port (to test connection errors).
///
/// Crea un archivo de configuración temporal con un puerto válido pero sin servidor (para probar errores de conexión).
fn create_no_server_config() -> NamedTempFile {
    let config = r#"
connection:
  mode: tcp
  host: "127.0.0.1"
  port: 12345
"#;
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), config).unwrap();
    file
}

/// Creates a malformed YAML configuration file.
///
/// Crea un archivo de configuración YAML malformado.
fn create_malformed_config() -> NamedTempFile {
    let config = r#"
connection:
  mode: tcp
  host: "127.0.0.1"
  port: 502
  invalid: [unclosed
"#;
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), config).unwrap();
    file
}

// -----------------------------------------------------------------------------
// Success cases for all register types
// -----------------------------------------------------------------------------

/// Test reading holding registers.
///
/// Prueba la lectura de registros holding.
#[test]
fn test_cli_read_holding_register() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("holding")
        .arg("-a")
        .arg("0")
        .arg("-n")
        .arg("1")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Registers read"));
    server.kill().expect("Failed to kill server");
}

/// Test reading input registers.
///
/// Prueba la lectura de registros input.
#[test]
fn test_cli_read_input_register() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("input")
        .arg("-a")
        .arg("10")
        .arg("-n")
        .arg("2")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Registers read"));
    server.kill().expect("Failed to kill server");
}

/// Test reading coils.
///
/// Prueba la lectura de coils.
#[test]
fn test_cli_read_coils() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("coil")
        .arg("-a")
        .arg("5")
        .arg("-n")
        .arg("3")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Registers read"));
    server.kill().expect("Failed to kill server");
}

/// Test reading discrete inputs.
///
/// Prueba la lectura de entradas discretas.
#[test]
fn test_cli_read_discrete_inputs() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("discrete")
        .arg("-a")
        .arg("8")
        .arg("-n")
        .arg("4")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Registers read"));
    server.kill().expect("Failed to kill server");
}

/// Test writing a coil.
///
/// Prueba la escritura de una coil.
#[test]
fn test_cli_write_coil() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("coil")
        .arg("-a")
        .arg("5")
        .arg("-v")
        .arg("1")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Value 1 written"));
    server.kill().expect("Failed to kill server");
}

/// Test writing a holding register.
///
/// Prueba la escritura de un registro holding.
#[test]
fn test_cli_write_holding_register() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("holding")
        .arg("-a")
        .arg("10")
        .arg("-v")
        .arg("12345")
        .arg("-c")
        .arg(config.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Value 12345 written"));
    server.kill().expect("Failed to kill server");
}

// -----------------------------------------------------------------------------
// Error cases
// -----------------------------------------------------------------------------

/// Test unsupported register type.
///
/// Prueba un tipo de registro no soportado.
#[test]
fn test_cli_unsupported_register_type() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-t")
        .arg("invalid")
        .arg("-a")
        .arg("0")
        .arg("-c")
        .arg(config.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("Unsupported register type"));
    server.kill().expect("Failed to kill server");
}

/// Test register address out of range (beyond the simulator's 100 registers).
///
/// Prueba una dirección de registro fuera de rango (más allá de los 100 registros del simulador).
#[test]
fn test_cli_address_out_of_range() {
    let (port, mut server) = start_slave_server();
    let config = create_temp_config(port);
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-a")
        .arg("200")
        .arg("-n")
        .arg("1")
        .arg("-c")
        .arg(config.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("Modbus exception"));
    server.kill().expect("Failed to kill server");
}

/// Test connection error (no server running on the port).
///
/// Prueba error de conexión (sin servidor en el puerto).
#[test]
fn test_cli_connection_error() {
    let config = create_no_server_config();
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-a")
        .arg("0")
        .arg("-c")
        .arg(config.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("Connection error"));
}

/// Test malformed configuration file.
///
/// Prueba archivo de configuración malformado.
#[test]
fn test_cli_malformed_config() {
    let config = create_malformed_config();
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("read")
        .arg("-a")
        .arg("0")
        .arg("-c")
        .arg(config.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("Configuration error"));
}

/// Test monitor command help.
///
/// Prueba la ayuda del comando monitor.
#[test]
fn test_cli_monitor_help() {
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("monitor").arg("-h").assert().success();
}

/// Test serve command help.
///
/// Prueba la ayuda del comando serve.
#[test]
fn test_cli_serve_help() {
    let mut cmd = Command::cargo_bin("modoc").unwrap();
    cmd.arg("serve").arg("-h").assert().success();
}
