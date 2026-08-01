use crate::core::{ModocError, Parity};
use tokio_modbus::client::rtu;
use tokio_modbus::Slave;
use tokio_serial::{DataBits, Parity as SerialParity, SerialPortBuilderExt, StopBits};

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
                "Número de bits de datos inválido".to_string(),
            ))
        }
    };
    let stop_bits = match stop_bits {
        1 => StopBits::One,
        2 => StopBits::Two,
        _ => {
            return Err(ModocError::Config(
                "Número de bits de parada inválido".to_string(),
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
