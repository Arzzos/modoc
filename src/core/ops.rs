//! Core operations for reading and writing Modbus registers.
//!
//! Provides high-level functions that abstract the underlying Modbus protocol details.
//! These functions handle configuration loading, client creation, and protocol-specific logic.

//! Operaciones principales para leer y escribir registros Modbus.
//!
//! Proporciona funciones de alto nivel que abstraen los detalles del protocolo Modbus subyacente.
//! Estas funciones manejan la carga de configuración, la creación del cliente y la lógica específica del protocolo.

use crate::core::error::Result;
use crate::core::ModocError;
use crate::protocol::create_client;

/// Default Modbus slave ID used when not specified in configuration.
///
/// ID de esclavo Modbus predeterminado utilizado cuando no se especifica en la configuración.
const DEFAULT_SLAVE_ID: u8 = 1;

/// Reads one or more registers from a Modbus device.
///
/// # Arguments
/// * `config_path` - Path to the YAML configuration file
/// * `register_type` - Type of register: "holding", "input", "coil", or "discrete"
/// * `address` - Starting address (0-based)
/// * `count` - Number of registers to read
///
/// # Returns
/// A vector of u16 values read from the device.
///
/// # Errors
/// Returns `ModocError` if the configuration is invalid, connection fails,
/// or the register type is not supported.
///
/// # Example
/// ```
/// let values = read_registers(&Path::new("config.yaml"), "holding", 0, 10).await?;
/// ```
///
/// Lee uno o más registros de un dispositivo Modbus.
///
/// # Argumentos
/// * `config_path` - Ruta al archivo de configuración YAML
/// * `register_type` - Tipo de registro: "holding", "input", "coil" o "discrete"
/// * `address` - Dirección de inicio (base 0)
/// * `count` - Número de registros a leer
///
/// # Retorna
/// Un vector de valores u16 leídos del dispositivo.
///
/// # Errores
/// Retorna `ModocError` si la configuración es inválida, falla la conexión,
/// o el tipo de registro no es soportado.
///
/// # Ejemplo
/// ```
/// let values = read_registers(&Path::new("config.yaml"), "holding", 0, 10).await?;
/// ```
pub async fn read_registers(
    config_path: &std::path::Path,
    register_type: &str,
    address: u16,
    count: u16,
) -> Result<Vec<u16>> {
    let config = crate::core::types::load_config(config_path)
        .map_err(|e| ModocError::Config(e.to_string()))?;
    let mut client = create_client(&config.connection, DEFAULT_SLAVE_ID).await?;

    let values = match register_type {
        "holding" => client
            .read_holding_registers(address, count)
            .await
            .map_err(ModocError::Modbus)?
            .map_err(ModocError::ModbusException)?,
        "input" => client
            .read_input_registers(address, count)
            .await
            .map_err(ModocError::Modbus)?
            .map_err(ModocError::ModbusException)?,
        "coil" => {
            let coils = client
                .read_coils(address, count)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?;
            coils.into_iter().map(|b| if b { 1 } else { 0 }).collect()
        }
        "discrete" => {
            let disc = client
                .read_discrete_inputs(address, count)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?;
            disc.into_iter().map(|b| if b { 1 } else { 0 }).collect()
        }
        _ => {
            return Err(ModocError::UnsupportedRegisterType(
                register_type.to_string(),
            ))
        }
    };

    Ok(values)
}

/// Writes a value to a Modbus register.
///
/// # Arguments
/// * `config_path` - Path to the YAML configuration file
/// * `register_type` - Type of register: "holding" or "coil"
/// * `address` - Address to write to (0-based)
/// * `value` - Value to write (for coils, 0 = OFF, non-zero = ON)
///
/// # Errors
/// Returns `ModocError` if the configuration is invalid, connection fails,
/// or the register type is not supported for writing.
///
/// Escribe un valor en un registro Modbus.
///
/// # Argumentos
/// * `config_path` - Ruta al archivo de configuración YAML
/// * `register_type` - Tipo de registro: "holding" o "coil"
/// * `address` - Dirección a escribir (base 0)
/// * `value` - Valor a escribir (para coils, 0 = OFF, distinto de cero = ON)
///
/// # Errores
/// Retorna `ModocError` si la configuración es inválida, falla la conexión,
/// o el tipo de registro no es soportado para escritura.
pub async fn write_register(
    config_path: &std::path::Path,
    register_type: &str,
    address: u16,
    value: u16,
) -> Result<()> {
    let config = crate::core::types::load_config(config_path)
        .map_err(|e| ModocError::Config(e.to_string()))?;
    let mut client = create_client(&config.connection, DEFAULT_SLAVE_ID).await?;

    match register_type {
        "holding" => {
            client
                .write_single_register(address, value)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?;
        }
        "coil" => {
            let val = value != 0;
            client
                .write_single_coil(address, val)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?;
        }
        _ => {
            return Err(ModocError::UnsupportedRegisterType(
                register_type.to_string(),
            ))
        }
    }
    Ok(())
}