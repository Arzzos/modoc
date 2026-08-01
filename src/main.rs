//! Modoc - A professional CLI tool for interacting with, monitoring, and simulating Modbus communications.
//!
//! This application provides three main functionalities:
//! - `read`: Read or write Modbus registers (Holding, Input, Coils, Discrete)
//! - `monitor`: Real-time dashboard with historical trend visualization
//! - `serve`: Virtual Modbus slave simulator for testing without hardware
//!
//! # Example
//! ```bash
//! # Read 10 holding registers starting at address 0
//! modoc read -a 0 -n 10
//!
//! # Monitor register 100 with 500ms interval
//! modoc monitor -a 100 -i 500
//!
//! # Start a TCP slave simulator on port 502
//! modoc serve -e 502 -m tcp
//! ```

//! Modoc - Una herramienta CLI profesional para interactuar, monitorear y simular comunicaciones Modbus.
//!
//! Esta aplicación proporciona tres funcionalidades principales:
//! - `read`: Leer o escribir registros Modbus (Holding, Input, Coils, Discrete)
//! - `monitor`: Dashboard en tiempo real con visualización de tendencia histórica
//! - `serve`: Simulador de esclavo Modbus virtual para pruebas sin hardware
//!
//! # Ejemplo
//! ```bash
//! # Leer 10 registros holding comenzando en la dirección 0
//! modoc read -a 0 -n 10
//!
//! # Monitorear el registro 100 con intervalo de 500ms
//! modoc monitor -a 100 -i 500
//!
//! # Iniciar un simulador esclavo TCP en el puerto 502
//! modoc serve -e 502 -m tcp
//! ```

mod cli;
mod core;
mod protocol;
mod simulator;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use tracing::info;

/// Main entry point for the Modoc application.
///
/// Parses command-line arguments, initializes logging, and dispatches
/// to the appropriate subcommand handler.
///
/// Punto de entrada principal para la aplicación Modoc.
///
/// Analiza los argumentos de línea de comandos, inicializa el registro
/// y despacha al manejador de subcomando apropiado.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("modoc=info")
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Read {
            register_type,
            address,
            count,
            value,
            config,
        } => {
            info!("Ejecutando comando 'read'");
            if let Some(val) = value {
                core::ops::write_register(&config, &register_type, address, val).await?;
                println!(
                    "Valor {} escrito en dirección {} (tipo: {})",
                    val, address, register_type
                );
            } else {
                let values =
                    core::ops::read_registers(&config, &register_type, address, count).await?;
                println!("Registros leídos (tipo: {}):", register_type);
                for (i, v) in values.iter().enumerate() {
                    println!("  [{}] = {}", address + i as u16, v);
                }
            }
        }
        Commands::Monitor {
            address,
            interval,
            config,
        } => {
            info!(
                "Iniciando monitor en registro {} cada {} ms",
                address, interval
            );
            ui::run_dashboard(&config, address, interval).await?;
        }
        Commands::Serve { endpoint, mode } => {
            info!(
                "Iniciando servidor/simulador Modbus {} en {}",
                mode, endpoint
            );
            simulator::run_slave(&endpoint, &mode).await?;
        }
    }

    Ok(())
}