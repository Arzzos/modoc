pub mod modbus_rtu;
pub mod modbus_tcp;

// DEV's:
// Capa de abstracción de hardware. Aquí se maneja la conexión con el metal (puerto serie)
// y sockets TCP usando tokio-modbus. Ningún código de UI debe entrar aquí.