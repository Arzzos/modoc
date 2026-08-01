use crate::core::error::Result;
use crate::core::ModocError;
use crate::protocol::create_client;

const DEFAULT_SLAVE_ID: u8 = 1;

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
        "holding" => {
            client
                .read_holding_registers(address, count)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?
        }
        "input" => {
            client
                .read_input_registers(address, count)
                .await
                .map_err(ModocError::Modbus)?
                .map_err(ModocError::ModbusException)?
        }
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