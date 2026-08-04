//! Core module of the Modoc application.
//!
//! Contains the fundamental types, errors, and operations for Modbus communication.
//! This module provides the main building blocks used by the CLI, UI, and protocol layers.

//! Módulo principal de la aplicación Modoc.
//!
//! Contiene los tipos fundamentales, errores y operaciones para la comunicación Modbus.
//! Este módulo proporciona los bloques de construcción principales utilizados por las capas CLI, UI y de protocolo.

pub mod error;
pub mod ops;
pub mod types;

pub use error::ModocError;
pub use ops::read_registers;
pub use types::*;
