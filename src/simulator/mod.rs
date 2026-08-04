//! Modbus slave simulator module.
//!
//! Provides a virtual Modbus slave for testing purposes without requiring real hardware.
//! Currently supports TCP mode with plans for RTU support.

//! Módulo simulador de esclavo Modbus.
//!
//! Proporciona un esclavo Modbus virtual para propósitos de prueba sin necesidad de hardware real.
//! Actualmente soporta modo TCP con planes para soporte RTU.

pub mod slave;
pub use slave::run_slave;
