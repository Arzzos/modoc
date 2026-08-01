mod cli;
mod core;
mod protocol;
mod simulator;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use tracing::info;
use tracing_subscriber;

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
