//! Modbus protocol implementation module.
//!
//! Provides client creation and communication abstractions for both TCP and RTU modes.
//! This module handles the low-level details of establishing connections and
//! managing the communication with Modbus devices.

//! Módulo de implementación del protocolo Modbus.
//!
//! Proporciona abstracciones para la creación de clientes y la comunicación en modos TCP y RTU.
//! Este módulo maneja los detalles de bajo nivel para establecer conexiones y
//! gestionar la comunicación con dispositivos Modbus.

pub mod modbus_rtu;
pub mod modbus_tcp;

use crate::core::ConnectionConfig;
use crate::core::ModocError;
use std::marker::Unpin;
use tokio_modbus::prelude::{Reader, Writer};

/// Trait combining Reader and Writer traits with Unpin and Send.
///
/// Used as a unified interface for both TCP and RTU clients.
///
/// Trait que combina los traits Reader y Writer con Unpin y Send.
///
/// Utilizado como una interfaz unificada para clientes TCP y RTU.
pub trait ModbusClient: Reader + Writer + Unpin + Send {}
impl<T: Reader + Writer + Unpin + Send> ModbusClient for T {}

/// Creates a Modbus client based on the provided configuration.
///
/// # Arguments
/// * `config` - Connection configuration (TCP or RTU)
/// * `slave_id` - Slave ID to use for the connection
///
/// # Returns
/// A boxed client implementing `ModbusClient`, or a `ModocError` if creation fails.
///
/// # Errors
/// Returns `ModocError` if the connection cannot be established.
///
/// Crea un cliente Modbus basado en la configuración proporcionada.
///
/// # Argumentos
/// * `config` - Configuración de conexión (TCP o RTU)
/// * `slave_id` - ID de esclavo a utilizar para la conexión
///
/// # Retorna
/// Un cliente encapsulado en un box que implementa `ModbusClient`, o un `ModocError` si falla la creación.
///
/// # Errores
/// Retorna `ModocError` si la conexión no puede ser establecida.
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