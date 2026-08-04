//! Implementation of a simple Modbus TCP slave.
//!
//! Responds to read/write requests for holding registers, coils, discrete inputs,
//! and input registers. All data is stored in memory with default values.
//!
//! Functions supported:
//! - 0x01: Read Coils
//! - 0x02: Read Discrete Inputs
//! - 0x03: Read Holding Registers
//! - 0x04: Read Input Registers
//! - 0x05: Write Single Coil
//! - 0x06: Write Single Register

//! Implementación de un esclavo Modbus TCP simple.
//!
//! Responde a peticiones de lectura/escritura para holding registers, coils,
//! discrete inputs e input registers. Todos los datos se almacenan en memoria
//! con valores predeterminados.
//!
//! Funciones soportadas:
//! - 0x01: Leer Coils
//! - 0x02: Leer Discrete Inputs
//! - 0x03: Leer Holding Registers
//! - 0x04: Leer Input Registers
//! - 0x05: Escribir Coil Simple
//! - 0x06: Escribir Registro Simple

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info};

/// Starts the slave server according to the specified mode and endpoint.
///
/// # Arguments
/// * `endpoint` - TCP port (e.g., "502") or serial port name (not implemented for RTU)
/// * `mode` - "tcp" or "rtu" (only TCP is implemented)
///
/// # Returns
/// A Result indicating success or failure.
///
/// Inicia el servidor esclavo según el modo y endpoint especificados.
///
/// # Argumentos
/// * `endpoint` - Puerto TCP (ej: "502") o nombre de puerto serie (no implementado para RTU)
/// * `mode` - "tcp" o "rtu" (solo TCP implementado)
///
/// # Retorna
/// Un Result que indica éxito o fallo.
pub async fn run_slave(endpoint: &str, mode: &str) -> Result<()> {
    match mode {
        "tcp" => run_tcp_slave(endpoint).await,
        "rtu" => {
            // RTU mode would require a serial server, left pending.
            // El modo RTU requeriría un servidor serie, lo dejamos pendiente.
            eprintln!("RTU mode not supported in the simulator yet.");
            Ok(())
        }
        _ => {
            eprintln!("Unsupported mode: {}", mode);
            Ok(())
        }
    }
}

/// Runs the TCP server for the Modbus slave simulation.
///
/// Ejecuta el servidor TCP para la simulación del esclavo Modbus.
async fn run_tcp_slave(port: &str) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await
        .with_context(|| format!("Failed to bind to {}", addr))?;
    info!("Modbus TCP simulator listening on {}", addr);

    // State: 100 holding registers, 100 coils, 100 discrete inputs, 100 input registers.
    // All initialized with predictable values (address-based for easy testing).
    // Estado: 100 holding registers, 100 coils, 100 discrete inputs, 100 input registers.
    // Todos inicializados con valores predecibles (basados en la dirección para facilitar pruebas).
    let holding_registers = Arc::new(Mutex::new((0..100).map(|i| i as u16).collect()));
    let coils = Arc::new(Mutex::new((0..100).map(|i| i % 2 == 0).collect()));
    let discrete_inputs = Arc::new(Mutex::new((0..100).map(|i| i % 3 == 0).collect()));
    let input_registers = Arc::new(Mutex::new((0..100).map(|i| (i * 10) as u16).collect()));

    loop {
        let (stream, _) = listener.accept().await?;
        let holding_registers = holding_registers.clone();
        let coils = coils.clone();
        let discrete_inputs = discrete_inputs.clone();
        let input_registers = input_registers.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, holding_registers, coils, discrete_inputs, input_registers).await {
                error!("Client error: {}", e);
            }
        });
    }
}

/// Handles an individual client connection.
///
/// Manages the Modbus request/response cycle for a single client,
/// parsing requests and returning appropriate responses or exceptions.
///
/// Maneja una conexión de cliente individual.
///
/// Gestiona el ciclo de solicitud/respuesta Modbus para un solo cliente,
/// analizando las solicitudes y devolviendo respuestas apropiadas o excepciones.
async fn handle_client(
    mut stream: TcpStream,
    holding_registers: Arc<Mutex<Vec<u16>>>,
    coils: Arc<Mutex<Vec<bool>>>,
    discrete_inputs: Arc<Mutex<Vec<bool>>>,
    input_registers: Arc<Mutex<Vec<u16>>>,
) -> Result<()> {
    let mut buf = [0u8; 260];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break; // connection closed by client
        }

        // Minimum Modbus TCP request size: 8 bytes (header + function code)
        if n < 8 {
            continue; // ignore incomplete requests
        }

        // Check the length field (bytes 4-5) indicates the expected payload size
        let expected_len = u16::from_be_bytes([buf[4], buf[5]]) as usize + 6; // 6 bytes header
        if n < expected_len {
            continue; // wait for more data (should not happen with a single read, but keep safety)
        }

        let unit_id = buf[6];
        let function = buf[7];

        match function {
            0x01 => {
                // Read Coils
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
                let quantity = u16::from_be_bytes([buf[10], buf[11]]);

                let coils_lock = coils.lock().await;
                if start_addr as usize + quantity as usize > coils_lock.len() || quantity == 0 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }

                let byte_count = ((quantity + 7) / 8) as u8;
                let len = (byte_count as u16) + 3; // unit_id + function + byte_count + data
                let mut response = Vec::with_capacity(6 + 1 + 1 + 1 + byte_count as usize);
                response.extend_from_slice(&buf[0..4]);
                response.extend_from_slice(&len.to_be_bytes());
                response.push(unit_id);
                response.push(0x01);
                response.push(byte_count);

                let mut bit_index = 0;
                let mut current_byte = 0;
                for i in 0..quantity {
                    let idx = start_addr as usize + i as usize;
                    if coils_lock[idx] {
                        current_byte |= 1 << bit_index;
                    }
                    bit_index += 1;
                    if bit_index == 8 || i == quantity - 1 {
                        response.push(current_byte);
                        current_byte = 0;
                        bit_index = 0;
                    }
                }
                drop(coils_lock);
                stream.write_all(&response).await?;
            }
            0x02 => {
                // Read Discrete Inputs
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
                let quantity = u16::from_be_bytes([buf[10], buf[11]]);

                let disc_lock = discrete_inputs.lock().await;
                if start_addr as usize + quantity as usize > disc_lock.len() || quantity == 0 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }

                let byte_count = ((quantity + 7) / 8) as u8;
                let len = (byte_count as u16) + 3;
                let mut response = Vec::with_capacity(6 + 1 + 1 + 1 + byte_count as usize);
                response.extend_from_slice(&buf[0..4]);
                response.extend_from_slice(&len.to_be_bytes());
                response.push(unit_id);
                response.push(0x02);
                response.push(byte_count);

                let mut bit_index = 0;
                let mut current_byte = 0;
                for i in 0..quantity {
                    let idx = start_addr as usize + i as usize;
                    if disc_lock[idx] {
                        current_byte |= 1 << bit_index;
                    }
                    bit_index += 1;
                    if bit_index == 8 || i == quantity - 1 {
                        response.push(current_byte);
                        current_byte = 0;
                        bit_index = 0;
                    }
                }
                drop(disc_lock);
                stream.write_all(&response).await?;
            }
            0x03 => {
                // Read Holding Registers
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
                let quantity = u16::from_be_bytes([buf[10], buf[11]]);

                let regs = holding_registers.lock().await;
                if start_addr as usize + quantity as usize > regs.len() || quantity == 0 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }

                let byte_count = (quantity * 2) as u8;
                let len = (byte_count as u16) + 3;
                let mut response = Vec::with_capacity(6 + 1 + 1 + 1 + byte_count as usize);
                response.extend_from_slice(&buf[0..4]);
                response.extend_from_slice(&len.to_be_bytes());
                response.push(unit_id);
                response.push(0x03);
                response.push(byte_count);
                for i in 0..quantity {
                    let val = regs[start_addr as usize + i as usize];
                    response.extend_from_slice(&val.to_be_bytes());
                }
                drop(regs);
                stream.write_all(&response).await?;
            }
            0x04 => {
                // Read Input Registers
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
                let quantity = u16::from_be_bytes([buf[10], buf[11]]);

                let regs = input_registers.lock().await;
                if start_addr as usize + quantity as usize > regs.len() || quantity == 0 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }

                let byte_count = (quantity * 2) as u8;
                let len = (byte_count as u16) + 3;
                let mut response = Vec::with_capacity(6 + 1 + 1 + 1 + byte_count as usize);
                response.extend_from_slice(&buf[0..4]);
                response.extend_from_slice(&len.to_be_bytes());
                response.push(unit_id);
                response.push(0x04);
                response.push(byte_count);
                for i in 0..quantity {
                    let val = regs[start_addr as usize + i as usize];
                    response.extend_from_slice(&val.to_be_bytes());
                }
                drop(regs);
                stream.write_all(&response).await?;
            }
            0x05 => {
                // Write Single Coil
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let addr = u16::from_be_bytes([buf[8], buf[9]]);
                let value = u16::from_be_bytes([buf[10], buf[11]]);
                if value != 0x0000 && value != 0xFF00 {
                    let response = build_exception_response(unit_id, 0x03);
                    stream.write_all(&response).await?;
                    continue;
                }
                let mut coils_lock = coils.lock().await;
                if addr as usize >= coils_lock.len() {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                coils_lock[addr as usize] = value == 0xFF00;
                drop(coils_lock);
                stream.write_all(&buf[0..12]).await?;
            }
            0x06 => {
                // Write Single Register
                if n < 12 {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                let addr = u16::from_be_bytes([buf[8], buf[9]]);
                let value = u16::from_be_bytes([buf[10], buf[11]]);
                let mut regs = holding_registers.lock().await;
                if addr as usize >= regs.len() {
                    let response = build_exception_response(unit_id, 0x02);
                    stream.write_all(&response).await?;
                    continue;
                }
                regs[addr as usize] = value;
                drop(regs);
                stream.write_all(&buf[0..12]).await?;
            }
            _ => {
                // Unsupported function
                let response = build_exception_response(unit_id, 0x01);
                stream.write_all(&response).await?;
            }
        }
    }
    Ok(())
}

/// Builds a Modbus exception response frame.
///
/// # Arguments
/// * `unit_id` - The unit identifier
/// * `exception_code` - The Modbus exception code
///
/// # Returns
/// A byte vector containing the properly formatted exception response.
///
/// Construye una trama de respuesta de excepción Modbus.
///
/// # Argumentos
/// * `unit_id` - El identificador de unidad
/// * `exception_code` - El código de excepción Modbus
///
/// # Retorna
/// Un vector de bytes que contiene la respuesta de excepción correctamente formateada.
pub fn build_exception_response(unit_id: u8, exception_code: u8) -> Vec<u8> {
    let mut resp = Vec::with_capacity(9);
    // Transaction ID = 0, Protocol ID = 0, Length = 3
    resp.extend_from_slice(&[0, 0, 0, 0, 0, 3]);
    resp.push(unit_id);
    resp.push(0x83); // function 0x03 with exception bit
    resp.push(exception_code);
    resp
}