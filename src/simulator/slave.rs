//! Implementación de un esclavo Modbus TCP simple.
//!
//! Responde a peticiones de lectura de holding registers (función 0x03).
//! Los registros se almacenan en memoria y se pueden modificar mediante escrituras
//! (aunque en esta versión solo se soporta lectura).

use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info};

/// Inicia el servidor esclavo según el modo y endpoint especificados.
///
/// # Argumentos
/// * `endpoint` - Puerto TCP (ej: "502") o nombre de puerto serie (no implementado para RTU).
/// * `mode` - "tcp" o "rtu" (solo TCP implementado).
pub async fn run_slave(endpoint: &str, mode: &str) -> Result<()> {
    match mode {
        "tcp" => run_tcp_slave(endpoint).await,
        "rtu" => {
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

/// Ejecuta el servidor TCP.
async fn run_tcp_slave(port: &str) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Simulador Modbus TCP escuchando en {}", addr);

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

/// Maneja una conexión de cliente individual.
async fn handle_client(mut stream: TcpStream, registers: Arc<Mutex<Vec<u16>>>) -> Result<()> {
    let mut buf = [0u8; 256];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break; // conexión cerrada por el cliente
        }

        // Parsear petición Modbus (solo soportamos función 0x03: read holding registers).
        // Estructura: [transaction_id(2), protocol_id(2), length(2), unit_id(1), function(1), start_addr(2), quantity(2)]
        if n < 8 {
            // Petición demasiado corta, ignorar.
            continue;
        }
        let function = buf[7];
        if function != 0x03 {
            // Enviar excepción: función no soportada (0x01).
            let response = build_exception_response(&buf[0..8], 0x01);
            stream.write_all(&response).await?;
            continue;
        }

        let start_addr = u16::from_be_bytes([buf[8], buf[9]]);
        let quantity = u16::from_be_bytes([buf[10], buf[11]]);

        // Validar rango.
        let regs = registers.lock().await;
        if start_addr as usize + quantity as usize > regs.len() {
            // Excepción: dirección de datos ilegal (0x02).
            let response = build_exception_response(&buf[0..8], 0x02);
            stream.write_all(&response).await?;
            continue;
        }

        // Construir respuesta.
        let mut response = Vec::with_capacity(2 + 2 + 2 + 1 + 1 + quantity as usize * 2);
        // Copiar cabecera (transaction ID, protocol ID, length se recalculará).
        response.extend_from_slice(&buf[0..6]);
        let byte_count = (quantity * 2) as u8;
        let len = (byte_count + 3) as u16; // unit_id + function + byte_count + datos
        response[4..6].copy_from_slice(&len.to_be_bytes());

        // Unit ID, Function, Byte Count.
        response.push(buf[6]); // unit_id
        response.push(0x03); // función
        response.push(byte_count);

        // Datos de los registros.
        for i in 0..quantity {
            let val = regs[start_addr as usize + i as usize];
            response.extend_from_slice(&val.to_be_bytes());
        }

        drop(regs);
        stream.write_all(&response).await?;
    }
    Ok(())
}

/// Construye una trama de respuesta de excepción Modbus.
fn build_exception_response(header: &[u8], exception_code: u8) -> Vec<u8> {
    let mut resp = Vec::with_capacity(9);
    resp.extend_from_slice(&header[0..6]); // transaction, protocol, length (se sobreescribirá)
    let len: u16 = 3; // unit_id + function + exception
    resp[4..6].copy_from_slice(&len.to_be_bytes());
    resp.push(header[6]); // unit_id
    resp.push(0x83); // función 0x03 con bit de excepción
    resp.push(exception_code);
    resp
}
