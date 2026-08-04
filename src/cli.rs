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
    /// Read or write Modbus registers (Holding, Input, Coils, Discrete).
    ///
    /// Lee o escribe registros Modbus (Holding, Input, Coils, Discrete).
    Read {
        /// Register type: holding, input, coil, discrete.
        ///
        /// Tipo de registro: holding, input, coil, discrete.
        #[arg(short = 't', long, default_value = "holding", help = "Register type (holding, input, coil, discrete)")]
        register_type: String,

        /// Starting address (0‑based according to Modbus standard).
        ///
        /// Dirección de inicio (0‑based según el estándar Modbus).
        #[arg(short = 'a', long, help = "Starting address (0‑based)")]
        address: u16,

        /// Number of registers to read (ignored if writing).
        ///
        /// Cantidad de registros a leer (ignorado si se escribe).
        #[arg(short = 'n', long, default_value = "1", help = "Number of registers to read")]
        count: u16,

        /// Value to write (if provided, performs a write instead of read).
        ///
        /// Valor a escribir (si se proporciona, se realiza escritura en lugar de lectura).
        #[arg(short = 'v', long, help = "Value to write (write mode enabled)")]
        value: Option<u16>,

        /// Path to the YAML configuration file.
        ///
        /// Ruta al archivo de configuración YAML.
        #[arg(short = 'c', long, default_value = "config.yaml", help = "Path to configuration file")]
        config: PathBuf,
    },

    /// Starts a real-time dashboard showing the evolution of a register.
    ///
    /// Inicia un dashboard en tiempo real que muestra la evolución de un registro.
    Monitor {
        /// Register address to monitor.
        ///
        /// Dirección del registro a monitorear.
        #[arg(short = 'a', long, help = "Register address to monitor")]
        address: u16,

        /// Sampling interval in milliseconds.
        ///
        /// Intervalo de muestreo en milisegundos.
        #[arg(short = 'i', long, default_value = "500", help = "Sampling interval (ms)")]
        interval: u64,

        /// Path to the configuration file.
        ///
        /// Ruta al archivo de configuración.
        #[arg(short = 'c', long, default_value = "config.yaml", help = "Path to configuration file")]
        config: PathBuf,
    },

    /// Acts as a virtual Modbus slave (simulator) for testing.
    ///
    /// Actúa como esclavo Modbus virtual (simulador) para pruebas.
    Serve {
        /// TCP port (e.g., "502") or serial port name (e.g., "COM3").
        ///
        /// Puerto TCP (ej. "502") o nombre del puerto serie (ej. "COM3").
        #[arg(short = 'e', long, default_value = "502", help = "Endpoint (port or serial device)")]
        endpoint: String,

        /// Communication mode: "tcp" or "rtu".
        ///
        /// Modo de comunicación: "tcp" o "rtu".
        #[arg(short = 'm', long, default_value = "tcp", help = "Mode (tcp or rtu)")]
        mode: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the read command parses correctly.
    ///
    /// Prueba que el comando read se analiza correctamente.
    #[test]
    fn verify_cli_parse_read() {
        let args = Cli::parse_from([
            "modoc", "read", "-a", "10", "-n", "5", "-c", "test.yaml"
        ]);
        match args.command {
            Commands::Read { address, count, config, register_type, value } => {
                assert_eq!(address, 10);
                assert_eq!(count, 5);
                assert_eq!(config, PathBuf::from("test.yaml"));
                assert_eq!(register_type, "holding");
                assert!(value.is_none());
            }
            _ => panic!("Expected Read command"),
        }
    }

    /// Tests that the read command with write value parses correctly.
    ///
    /// Prueba que el comando read con valor de escritura se analiza correctamente.
    #[test]
    fn verify_cli_parse_write() {
        let args = Cli::parse_from([
            "modoc", "read", "-a", "2", "-v", "123", "-t", "coil"
        ]);
        match args.command {
            Commands::Read { address, value, register_type, .. } => {
                assert_eq!(address, 2);
                assert_eq!(value, Some(123));
                assert_eq!(register_type, "coil");
            }
            _ => panic!("Expected Read command with write"),
        }
    }

    /// Tests that the monitor command parses correctly.
    ///
    /// Prueba que el comando monitor se analiza correctamente.
    #[test]
    fn verify_cli_parse_monitor() {
        let args = Cli::parse_from([
            "modoc", "monitor", "-a", "42", "-i", "100"
        ]);
        match args.command {
            Commands::Monitor { address, interval, .. } => {
                assert_eq!(address, 42);
                assert_eq!(interval, 100);
            }
            _ => panic!("Expected Monitor command"),
        }
    }

    /// Tests that the serve command parses correctly.
    ///
    /// Prueba que el comando serve se analiza correctamente.
    #[test]
    fn verify_cli_parse_serve() {
        let args = Cli::parse_from([
            "modoc", "serve", "-e", "5020", "-m", "tcp"
        ]);
        match args.command {
            Commands::Serve { endpoint, mode } => {
                assert_eq!(endpoint, "5020");
                assert_eq!(mode, "tcp");
            }
            _ => panic!("Expected Serve command"),
        }
    }
}