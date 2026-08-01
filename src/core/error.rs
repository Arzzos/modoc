use thiserror::Error;

pub type Result<T> = std::result::Result<T, ModocError>;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ModocError {
    #[error("Error de conexión: {0}")]
    Connection(String),

    #[error("Error de comunicación Modbus: {0}")]
    Modbus(#[from] tokio_modbus::Error),

    #[error("Error de puerto serie: {0}")]
    Serial(#[from] serialport::Error),

    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de configuración: {0}")]
    Config(String),

    #[error("Tipo de registro no soportado: {0}")]
    UnsupportedRegisterType(String),

    #[error("Valor fuera de rango")]
    ValueOutOfRange,

    #[error("Tiempo de espera agotado")]
    Timeout,

    #[error("Error de CRC en la trama Modbus")]
    CrcError,

    #[error("Excepción Modbus: {0:?}")]
    ModbusException(#[from] tokio_modbus::Exception),
}