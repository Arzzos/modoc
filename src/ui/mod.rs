//! Módulo de interfaz de usuario en terminal (TUI).
//!
//! Proporciona el dashboard interactivo para el comando `monitor`.

mod dashboard;
mod event;
mod widgets;

pub use dashboard::run_dashboard;
