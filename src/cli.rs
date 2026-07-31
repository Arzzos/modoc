use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "modoc", author, version, about = "Modbus Organism Designed Only for Coding")]
pub struct Cli {
    #[arg(short, long, default_value = "rtu")]
    pub mode: String,

    #[subcommand]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Read registers from an industrial device
    Read {
        #[arg(short, long)]
        port_or_ip: String,
        #[arg(short, long)]
        address: u16,
    },
    /// Monitor device registers in a live TUI dashboard
    Monitor {
        #[arg(short, long)]
        port_or_ip: String,
        #[arg(short, long)]
        address: u16,
    },
    /// Simulate a Modbus device (Slave/Server)
    Serve {
        #[arg(short, long, default_value = "502")]
        port: String,
    },
}