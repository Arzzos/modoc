# Modoc

A modern, ultra-fast, cross-platform CLI and TUI companion for Modbus TCP/RTU protocol inspection, real-time monitoring, and hardware simulation built in Rust.

---

## English Description

Modoc is structured using a decoupled modular architecture to ensure high performance and strict separation of concerns:
- `core/`: Central data models and robust industrial error management (`ModocError`).
- `protocol/`: Asynchronous Modbus TCP (Ethernet) and RTU (RS485/Serial) hardware abstraction layer.
- `ui/`: Terminal User Interface (TUI) layer built with `ratatui` for real-time streaming dashboards.
- `simulator/`: A virtual Modbus slave engine to mock industrial devices for local testing.

### Usage
```bash
cargo build --release
cargo run -- --help
```

## Descripción en Español
Modoc está estructurado utilizando una arquitectura modular desacoplada para garantizar un alto rendimiento y una estricta separación de responsabilidades:

- `core/`: Modelos de datos centrales y gestión robusta de errores industriales (ModocError).

- `protocol/`: Capa de abstracción de hardware para Modbus TCP (Ethernet) y RTU (RS485/Serie) asíncronos.

-  `ui/`: Interfaz de terminal (TUI) construida con ratatui para paneles de monitoreo en tiempo real.

- `simulator/`: Motor de simulación esclavo Modbus virtual para pruebas locales de software sin hardware real.

### Uso

```bash
cargo build --release
cargo run -- --help
```

License / Licencia
Dual-licensed under either MIT License or Apache License, Version 2.0.