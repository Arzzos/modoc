//! Definición de la interfaz de línea de comandos usando `clap`.
//!
//! Se estructuran los tres subcomandos principales: `read`, `monitor` y `serve`,
//! cada uno con sus argumentos y opciones específicas.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Estructura raíz de la CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum de subcomandos disponibles.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Lee o escribe registros Modbus (Holding, Input, Coils, Discrete).
    Read {
        /// Tipo de registro: holding, input, coil, discrete.
        #[arg(short, long, default_value = "holding")]
        register_type: String,

        /// Dirección de inicio (0-based según el estándar Modbus).
        #[arg(short, long)]
        address: u16,

        /// Cantidad de registros a leer (ignorado si se escribe).
        #[arg(short, long, default_value = "1")]
        count: u16,

        /// Valor a escribir (si se proporciona, se realiza escritura en lugar de lectura).
        #[arg(short, long)]
        value: Option<u16>,

        /// Ruta al archivo de configuración YAML.
        #[arg(short, long, default_value = "config.yaml")]
        config: PathBuf,
    },

    /// Inicia un dashboard en tiempo real que muestra la evolución de un registro.
    Monitor {
        /// Dirección del registro a monitorear.
        #[arg(short, long)]
        address: u16,

        /// Intervalo de muestreo en milisegundos.
        #[arg(short, long, default_value = "500")]
        interval: u64,

        /// Ruta al archivo de configuración.
        #[arg(short, long, default_value = "config.yaml")]
        config: PathBuf,
    },

    /// Actúa como esclavo Modbus virtual (simulador) para pruebas.
    Serve {
        /// Puerto TCP (ej. "502") o nombre del puerto serie (ej. "COM3").
        #[arg(short, long, default_value = "502")]
        endpoint: String,

        /// Modo de comunicación: "tcp" o "rtu".
        #[arg(short, long, default_value = "tcp")]
        mode: String,
    },
}
