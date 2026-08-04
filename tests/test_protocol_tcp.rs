//! Unit tests for TCP Modbus client connection.
//!
//! Tests connection errors, invalid addresses, and timeouts.
//! These tests do not require a real Modbus device.
//!
//! Pruebas unitarias para la conexión del cliente Modbus TCP.
//!
//! Prueba errores de conexión, direcciones inválidas y tiempos de espera.
//! Estas pruebas no requieren un dispositivo Modbus real.

use modoc::core::ModocError;
use modoc::protocol::modbus_tcp::connect_tcp;
use tokio::time::{timeout, Duration};

/// Tests connecting to an invalid address (non-existent IP).
///
/// Prueba la conexión a una dirección inválida (IP inexistente).
#[tokio::test]
async fn test_connect_tcp_invalid_address() {
    // Use a non-routable IP or an address that is unlikely to exist.
    let result = connect_tcp("192.0.2.0", 502, 1).await;
    assert!(matches!(result, Err(ModocError::Connection(_))));
}

/// Tests connecting to a valid local address where no server is listening.
///
/// Prueba la conexión a una dirección local válida donde no hay ningún servidor escuchando.
#[tokio::test]
async fn test_connect_tcp_no_server() {
    // Use a local port that is probably free (e.g., 12345).
    let result = timeout(Duration::from_secs(2), connect_tcp("127.0.0.1", 12345, 1)).await;
    assert!(result.is_err() || matches!(result.unwrap(), Err(ModocError::Connection(_))));
}

/// Tests parsing an invalid socket address.
///
/// Prueba el análisis de una dirección socket inválida.
#[tokio::test]
async fn test_connect_tcp_invalid_socket() {
    let result = connect_tcp("not-an-address", 502, 1).await;
    assert!(matches!(result, Err(ModocError::Connection(_))));
}
