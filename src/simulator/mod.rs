//! Simulador de esclavo Modbus.
//!
//! Permite emular un dispositivo Modbus para pruebas sin hardware real.

mod slave;
pub use slave::run_slave;
