mod cli;
mod core;
mod protocol;
mod simulator;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar el sistema de logs/trazas industriales (tracing)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Cli::parse();
    info!("Initializing Modoc core engine in [{}] mode", args.mode);

    match args.command {
        Commands::Read { target, address, count } => {
            info!("Executing Modoc Read Command");
            println!("Reading {} registers starting at address {} from {}...", count, address, target);
            // ToDo: Implementar la llamada a protocol::modbus_rtu o tcp aquí.
        }
        Commands::Monitor { target, address } => {
            info!("Spawning Live TUI Dashboard Context");
            println!("Connecting to {} to monitor address {} in real-time...", target, address);
            // ToDo: Aquí arrancará el loop asíncrono de ui::dashboard::start.
        }
        Commands::Serve { port } => {
            info!("Booting Up Virtual Modbus Server Environment");
            println!("Modoc virtual slave listening on port {}...", port);
            // ToDo: Invocar el simulador en simulator::slave::run_server.
        }
    }

    Ok(())
}