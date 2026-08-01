use std::net::SocketAddr;
use tokio_modbus::client::tcp::connect_slave;
use tokio_modbus::Slave;
use crate::core::ModocError;

pub async fn connect_tcp(
    host: &str,
    port: u16,
    slave_id: u8,
) -> Result<tokio_modbus::client::Context, ModocError> {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .map_err(|_| ModocError::Connection("Dirección socket inválida".to_string()))?;
    let client = connect_slave(addr, Slave(slave_id))
        .await
        .map_err(|e| ModocError::Connection(e.to_string()))?;
    Ok(client)
}