//! Unit tests for RTU (serial) Modbus client connection.
//!
//! Tests invalid parameters, such as unsupported data bits or stop bits.
//! These tests do not require actual serial hardware.
//!
//! Pruebas unitarias para la conexión del cliente Modbus RTU (serie).
//!
//! Prueba parámetros inválidos, como bits de datos o bits de parada no soportados.
//! Estas pruebas no requieren hardware serie real.

use modoc::core::{ModocError, Parity};
use modoc::protocol::modbus_rtu::connect_rtu;

/// Tests connecting with an invalid data bits value.
///
/// Prueba la conexión con un valor de bits de datos inválido.
#[tokio::test]
async fn test_connect_rtu_invalid_data_bits() {
    let result = connect_rtu("/dev/null", 9600, 4, 1, Parity::None, 1).await;
    assert!(matches!(result, Err(ModocError::Config(_))));
}

/// Tests connecting with an invalid stop bits value.
///
/// Prueba la conexión con un valor de bits de parada inválido.
#[tokio::test]
async fn test_connect_rtu_invalid_stop_bits() {
    let result = connect_rtu("/dev/null", 9600, 8, 3, Parity::None, 1).await;
    assert!(matches!(result, Err(ModocError::Config(_))));
}

/// Tests connecting with an invalid baud rate (not actually validated by serialport but we can still test).
///
/// Prueba la conexión con una velocidad en baudios inválida (no validada realmente por serialport, pero podemos probarlo).
#[tokio::test]
async fn test_connect_rtu_invalid_baud() {
    let result = connect_rtu("/dev/null", 999999, 8, 1, Parity::None, 1).await;
    // The serialport library may accept any baud rate, but if it fails we expect a Connection error.
    match result {
        Err(ModocError::Connection(_)) => {} // expected
        Err(ModocError::Config(_)) => {} // also possible if serialport rejects it
        _ => panic!("Expected Connection or Config error"),
    }
}