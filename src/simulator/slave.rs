//! Implementation of a simple Modbus TCP slave.
//!
//! Responds to read holding registers requests (function 0x03) with data stored in memory.
//! The slave maintains a set of registers that can be read but not modified in this version.

//! Implementación de un esclavo Modbus TCP simple.
//!
//! Responde a peticiones de lectura de holding registers (función 0x03) con datos almacenados en memoria.
//! El esclavo mantiene un conjunto de registros que pueden ser leídos pero no modificados en esta versión.

use anyhow::Result;
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
            eprintln!("Modo RTU no soportado en el simulador aún.");
            Ok(())
        }
        _ => {
            eprintln!("Modo no soportado: {}", mode);
            Ok(())
        }
    }
}

/// Runs the TCP server for the Modbus slave simulation.
///
/// Ejecuta el servidor TCP para la simulación del esclavo Modbus.
async fn run_tcp_slave(port: &str) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Simulador Modbus TCP escuchando en {}", addr);

    // State: 100 holding registers initialized to 0.
    // Estado: 100 holding registers inicializados a 0.
    let registers = Arc::new(Mutex::new(vec![0u16; 100]));

    loop {
        let (stream, _) = listener.accept().await?;
        let registers = registers.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, registers).await {
                error!("Error con cliente: {}", e);
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
async fn handle_client(mut stream: TcpStream, registers: Arc<Mutex<Vec<u16>>>) -> Result<()> {
    let mut buf = [0u8; 256];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break; // connection closed by client
        }

        // Parse Modbus request (only support function 0x03: read holding registers).
        // Structure: [transaction_id(2), protocol_id(2), length(2), unit_id(1), function(1), start_addr(2), quantity(2)]
        if n < 8 {
            // Request too short, ignore.
            continue;
        }
        let function = buf[7];
        if function != 0x03 {
            // Send exception: unsupported function (0x01).
            let response = build_exception_response(&buf[0..8], 0x01);
            stream.write_all(&response).await?;
            continue;
        }

        let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
        let quantity = u16::from_be_bytes([buf[10], buf[11]]);

        // Validate range.
        let regs = registers.lock().await;
        if start_addr as usize + quantity as usize > regs.len() {
            // Exception: illegal data address (0x02).
            let response = build_exception_response(&buf[0..8], 0x02);
            stream.write_all(&response).await?;
            continue;
        }

        // Build response.
        let mut response = Vec::with_capacity(2 + 2 + 2 + 1 + 1 + quantity as usize * 2);
        // Copy header (transaction ID, protocol ID, length will be recalculated).
        response.extend_from_slice(&buf[0..6]);
        let byte_count = (quantity * 2) as u8;
        let len = (byte_count + 3) as u16; // unit_id + function + byte_count + data
        response[4..6].copy_from_slice(&len.to_be_bytes());

        // Unit ID, Function, Byte Count.
        response.push(buf[6]); // unit_id
        response.push(0x03); // function
        response.push(byte_count);

        // Register data.
        for i in 0..quantity {
            let val = regs[start_addr as usize + i as usize];
            response.extend_from_slice(&val.to_be_bytes());
        }

        drop(regs);
        stream.write_all(&response).await?;
    }
    Ok(())
}

/// Builds a Modbus exception response frame.
///
/// # Arguments
/// * `header` - The request header (first 8 bytes) to copy transaction IDs from
/// * `exception_code` - The Modbus exception code
///
/// # Returns
/// A byte vector containing the properly formatted exception response.
///
/// Construye una trama de respuesta de excepción Modbus.
///
/// # Argumentos
/// * `header` - La cabecera de la solicitud (primeros 8 bytes) para copiar los IDs de transacción
/// * `exception_code` - El código de excepción Modbus
///
/// # Retorna
/// Un vector de bytes que contiene la respuesta de excepción correctamente formateada.
fn build_exception_response(header: &[u8], exception_code: u8) -> Vec<u8> {
    let mut resp = Vec::with_capacity(9);
    resp.extend_from_slice(&header[0..6]); // transaction, protocol, length (will be overwritten)
    let len: u16 = 3; // unit_id + function + exception
    resp[4..6].copy_from_slice(&len.to_be_bytes());
    resp.push(header[6]); // unit_id
    resp.push(0x83); // function 0x03 with exception bit
    resp.push(exception_code);
    resp
}