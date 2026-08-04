//! TCP (Ethernet) Modbus client implementation.
//!
//! Provides functions to establish a TCP connection and create a client
//! for communicating with Modbus devices over Ethernet.

//! Implementación del cliente Modbus TCP (Ethernet).
//!
//! Proporciona funciones para establecer una conexión TCP y crear un cliente
//! para comunicarse con dispositivos Modbus a través de Ethernet.

use crate::core::ModocError;
use std::net::SocketAddr;
use tokio_modbus::client::tcp::connect_slave;
use tokio_modbus::Slave;

/// Establishes a connection to a Modbus TCP device over Ethernet.
///
/// # Arguments
/// * `host` - Hostname or IP address of the Modbus device
/// * `port` - TCP port (usually 502)
/// * `slave_id` - Slave ID for the Modbus device (Unit ID)
///
/// # Returns
/// A Modbus client context ready for communication.
///
/// # Errors
/// Returns `ModocError` if the address is invalid or the connection fails.
///
/// Establece una conexión a un dispositivo Modbus TCP a través de Ethernet.
///
/// # Argumentos
/// * `host` - Nombre de host o dirección IP del dispositivo Modbus
/// * `port` - Puerto TCP (generalmente 502)
/// * `slave_id` - ID de esclavo para el dispositivo Modbus (Unit ID)
///
/// # Retorna
/// Un contexto de cliente Modbus listo para la comunicación.
///
/// # Errores
/// Retorna `ModocError` si la dirección es inválida o la conexión falla.
pub async fn connect_tcp(
    host: &str,
    port: u16,
    slave_id: u8,
) -> Result<tokio_modbus::client::Context, ModocError> {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .map_err(|_| ModocError::Connection("Invalid socket address".to_string()))?;
    let client = connect_slave(addr, Slave(slave_id))
        .await
        .map_err(|e| ModocError::Connection(e.to_string()))?;
    Ok(client)
}