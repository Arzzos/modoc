//! Terminal user interface (TUI) module.
//!
//! Provides the interactive dashboard for the `monitor` command.
//! This module handles all visual components and user interaction for the real-time monitoring feature.

//! Módulo de interfaz de usuario en terminal (TUI).
//!
//! Proporciona el dashboard interactivo para el comando `monitor`.
//! Este módulo maneja todos los componentes visuales y la interacción con el usuario para la función de monitoreo en tiempo real.

mod dashboard;
mod event;
mod widgets;

pub use dashboard::run_dashboard;