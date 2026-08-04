//! Error handling for the Modoc application.
//!
//! Defines the main error type and result type used throughout the application.
//! All Modbus-related errors are unified under a single enum for consistent handling.

//! Manejo de errores para la aplicación Modoc.
//!
//! Define el tipo de error principal y el tipo de resultado utilizado en toda la aplicación.
//! Todos los errores relacionados con Modbus se unifican en un solo enum para un manejo consistente.

use thiserror::Error;

/// Result type alias using ModocError.
///
/// Alias de resultado que utiliza ModocError.
pub type Result<T> = std::result::Result<T, ModocError>;

/// Main error type for the Modoc application.
///
/// This enum covers all possible error conditions that can occur during
/// Modbus communication, configuration, and I/O operations.
///
/// Tipo de error principal para la aplicación Modoc.
///
/// Este enum cubre todas las posibles condiciones de error que pueden ocurrir durante
/// la comunicación Modbus, configuración y operaciones de E/S.
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ModocError {
    /// Connection error (network, serial, etc.)
    ///
    /// Error de conexión (red, serie, etc.)
    #[error("Connection error: {0}")]
    Connection(String),

    /// Modbus protocol communication error
    ///
    /// Error de comunicación del protocolo Modbus
    #[error("Modbus communication error: {0}")]
    Modbus(#[from] tokio_modbus::Error),

    /// Serial port error
    ///
    /// Error del puerto serie
    #[error("Serial port error: {0}")]
    Serial(#[from] serialport::Error),

    /// Generic I/O error
    ///
    /// Error de E/S genérico
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error (invalid config file, missing fields, etc.)
    ///
    /// Error de configuración (archivo de configuración inválido, campos faltantes, etc.)
    #[error("Configuration error: {0}")]
    Config(String),

    /// Unsupported register type requested
    ///
    /// Tipo de registro no soportado solicitado
    #[error("Unsupported register type: {0}")]
    UnsupportedRegisterType(String),

    /// Value is out of valid range
    ///
    /// Valor fuera del rango válido
    #[error("Value out of range")]
    ValueOutOfRange,

    /// Operation timed out
    ///
    /// Tiempo de espera agotado
    #[error("Operation timed out")]
    Timeout,

    /// CRC error in Modbus frame
    ///
    /// Error de CRC en la trama Modbus
    #[error("CRC error in Modbus frame")]
    CrcError,

    /// Modbus exception response received from device
    ///
    /// Respuesta de excepción Modbus recibida del dispositivo
    #[error("Modbus exception: {0:?}")]
    ModbusException(#[from] tokio_modbus::Exception),
}
