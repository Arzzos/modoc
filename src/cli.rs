//! Command-line interface definition using `clap`.
//!
//! Structures the three main subcommands: `read`, `monitor`, and `serve`,
//! each with their specific arguments and options.

//! Definición de la interfaz de línea de comandos usando `clap`.
//!
//! Se estructuran los tres subcomandos principales: `read`, `monitor` y `serve`,
//! cada uno con sus argumentos y opciones específicas.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Root CLI structure.
///
/// Estructura raíz de la CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Enumeration of available subcommands.
///
/// Enum de subcomandos disponibles.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Reads or writes Modbus registers (Holding, Input, Coils, Discrete).
    ///
    /// Lee o escribe registros Modbus (Holding, Input, Coils, Discrete).
    Read {
        /// Register type: holding, input, coil, discrete.
        ///
        /// Tipo de registro: holding, input, coil, discrete.
        #[arg(short = 't', long, default_value = "holding")]
        register_type: String,

        /// Starting address (0-based according to Modbus standard).
        ///
        /// Dirección de inicio (0-based según el estándar Modbus).
        #[arg(short = 'a', long)]
        address: u16,

        /// Number of registers to read (ignored if writing).
        ///
        /// Cantidad de registros a leer (ignorado si se escribe).
        #[arg(short = 'n', long, default_value = "1")]
        count: u16,

        /// Value to write (if provided, performs a write instead of read).
        ///
        /// Valor a escribir (si se proporciona, se realiza escritura en lugar de lectura).
        #[arg(short = 'v', long)]
        value: Option<u16>,

        /// Path to the YAML configuration file.
        ///
        /// Ruta al archivo de configuración YAML.
        #[arg(short = 'c', long, default_value = "config.yaml")]
        config: PathBuf,
    },

    /// Starts a real-time dashboard showing the evolution of a register.
    ///
    /// Inicia un dashboard en tiempo real que muestra la evolución de un registro.
    Monitor {
        /// Register address to monitor.
        ///
        /// Dirección del registro a monitorear.
        #[arg(short = 'a', long)]
        address: u16,

        /// Sampling interval in milliseconds.
        ///
        /// Intervalo de muestreo en milisegundos.
        #[arg(short = 'i', long, default_value = "500")]
        interval: u64,

        /// Path to the configuration file.
        ///
        /// Ruta al archivo de configuración.
        #[arg(short = 'c', long, default_value = "config.yaml")]
        config: PathBuf,
    },

    /// Acts as a virtual Modbus slave (simulator) for testing.
    ///
    /// Actúa como esclavo Modbus virtual (simulador) para pruebas.
    Serve {
        /// TCP port (e.g., "502") or serial port name (e.g., "COM3").
        ///
        /// Puerto TCP (ej. "502") o nombre del puerto serie (ej. "COM3").
        #[arg(short = 'e', long, default_value = "502")]
        endpoint: String,

        /// Communication mode: "tcp" or "rtu".
        ///
        /// Modo de comunicación: "tcp" o "rtu".
        #[arg(short = 'm', long, default_value = "tcp")]
        mode: String,
    },
}