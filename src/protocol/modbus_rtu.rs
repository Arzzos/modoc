//! RTU (serial) Modbus client implementation.
//!
//! Provides functions to establish a serial connection and create an RTU client
//! for communicating with Modbus devices over RS-232/RS-485.

//! Implementación del cliente Modbus RTU (serie).
//!
//! Proporciona funciones para establecer una conexión serie y crear un cliente RTU
//! para comunicarse con dispositivos Modbus a través de RS-232/RS-485.

use crate::core::{ModocError, Parity};
use tokio_modbus::client::rtu;
use tokio_modbus::Slave;
use tokio_serial::{DataBits, Parity as SerialParity, SerialPortBuilderExt, StopBits};

/// Establishes a connection to a Modbus RTU device over a serial port.
///
/// # Arguments
/// * `port_name` - Serial port name (e.g., "COM3" on Windows, "/dev/ttyUSB0" on Linux)
/// * `baud_rate` - Baud rate (e.g., 9600, 19200, 115200)
/// * `data_bits` - Number of data bits (5, 6, 7, 8)
/// * `stop_bits` - Number of stop bits (1 or 2)
/// * `parity` - Parity setting (None, Odd, Even)
/// * `slave_id` - Slave ID for the Modbus device
///
/// # Returns
/// A Modbus client context ready for communication.
///
/// # Errors
/// Returns `ModocError` if the serial port cannot be opened or the configuration is invalid.
///
/// Establece una conexión a un dispositivo Modbus RTU a través de un puerto serie.
///
/// # Argumentos
/// * `port_name` - Nombre del puerto serie (ej., "COM3" en Windows, "/dev/ttyUSB0" en Linux)
/// * `baud_rate` - Velocidad en baudios (ej., 9600, 19200, 115200)
/// * `data_bits` - Número de bits de datos (5, 6, 7, 8)
/// * `stop_bits` - Número de bits de parada (1 o 2)
/// * `parity` - Configuración de paridad (None, Odd, Even)
/// * `slave_id` - ID de esclavo para el dispositivo Modbus
///
/// # Retorna
/// Un contexto de cliente Modbus listo para la comunicación.
///
/// # Errores
/// Retorna `ModocError` si el puerto serie no puede ser abierto o la configuración es inválida.
pub async fn connect_rtu(
    port_name: &str,
    baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
    parity: Parity,
    slave_id: u8,
) -> Result<tokio_modbus::client::Context, ModocError> {
    let data_bits = match data_bits {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        8 => DataBits::Eight,
        _ => {
            return Err(ModocError::Config(
                "Invalid data bits value (must be 5, 6, 7, or 8)".to_string(),
            ))
        }
    };
    let stop_bits = match stop_bits {
        1 => StopBits::One,
        2 => StopBits::Two,
        _ => {
            return Err(ModocError::Config(
                "Invalid stop bits value (must be 1 or 2)".to_string(),
            ))
        }
    };
    let parity = match parity {
        Parity::None => SerialParity::None,
        Parity::Odd => SerialParity::Odd,
        Parity::Even => SerialParity::Even,
    };

    let port = tokio_serial::new(port_name, baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .open_native_async()
        .map_err(|e| ModocError::Connection(e.to_string()))?;

    let client = rtu::attach_slave(port, Slave(slave_id));
    Ok(client)
}
