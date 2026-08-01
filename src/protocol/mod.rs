mod modbus_rtu;
mod modbus_tcp;

use crate::core::ConnectionConfig;
use crate::core::ModocError;
use tokio_modbus::prelude::*;
use std::marker::Unpin;

pub trait ModbusClient: Reader + Writer + Unpin + Send {}
impl<T: Reader + Writer + Unpin + Send> ModbusClient for T {}

pub async fn create_client(
    config: &ConnectionConfig,
    slave_id: u8,
) -> Result<Box<dyn ModbusClient>, ModocError> {
    match config {
        ConnectionConfig::Tcp { host, port } => {
            let client = modbus_tcp::connect_tcp(host, *port, slave_id).await?;
            Ok(Box::new(client))
        }
        ConnectionConfig::Rtu {
            serial_port,
            baud_rate,
            data_bits,
            stop_bits,
            parity,
        } => {
            let client = modbus_rtu::connect_rtu(
                serial_port,
                *baud_rate,
                *data_bits,
                *stop_bits,
                parity.clone(),
                slave_id,
            )
            .await?;
            Ok(Box::new(client))
        }
    }
}