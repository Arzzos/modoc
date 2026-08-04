//! Unit tests for the built-in Modbus TCP simulator.
//!
//! Tests the response building functions and exception handling logic.
//!
//! Pruebas unitarias para el simulador Modbus TCP integrado.
//!
//! Prueba las funciones de construcción de respuestas y la lógica de manejo de excepciones.

use modoc::simulator::slave::build_exception_response;

/// Tests building an exception response for illegal function (0x01).
///
/// Prueba la construcción de una respuesta de excepción para función ilegal (0x01).
#[test]
fn test_build_exception_response_illegal_function() {
    let unit_id = 0x01;
    let exception_code = 0x01;
    let response = build_exception_response(unit_id, exception_code);
    // Expected: [0,0,0,0,0,3, unit_id, 0x83, exception_code]
    assert_eq!(response.len(), 9);
    assert_eq!(&response[0..6], &[0, 0, 0, 0, 0, 3]);
    assert_eq!(response[6], unit_id);
    assert_eq!(response[7], 0x83);
    assert_eq!(response[8], exception_code);
}

/// Tests building an exception response for illegal data address (0x02).
///
/// Prueba la construcción de una respuesta de excepción para dirección de datos ilegal (0x02).
#[test]
fn test_build_exception_response_illegal_data_address() {
    let unit_id = 0x02;
    let exception_code = 0x02;
    let response = build_exception_response(unit_id, exception_code);
    assert_eq!(response.len(), 9);
    assert_eq!(&response[0..6], &[0, 0, 0, 0, 0, 3]);
    assert_eq!(response[6], unit_id);
    assert_eq!(response[7], 0x83);
    assert_eq!(response[8], exception_code);
}

/// Tests building an exception response for illegal data value (0x03).
///
/// Prueba la construcción de una respuesta de excepción para valor de datos ilegal (0x03).
#[test]
fn test_build_exception_response_illegal_data_value() {
    let unit_id = 0x03;
    let exception_code = 0x03;
    let response = build_exception_response(unit_id, exception_code);
    assert_eq!(response.len(), 9);
    assert_eq!(&response[0..6], &[0, 0, 0, 0, 0, 3]);
    assert_eq!(response[6], unit_id);
    assert_eq!(response[7], 0x83);
    assert_eq!(response[8], exception_code);
}