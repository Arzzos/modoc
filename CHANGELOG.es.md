# Registro de cambios

Todos los cambios notables de este proyecto se documentan en este archivo.

El formato se basa en [Keep a Changelog](https://keepachangelog.com/es/1.0.0/), y este proyecto sigue [Versionado Semántico](https://semver.org/lang/es/).

## [0.2.0] - 2026-08-04

### Añadido

- Comando `monitor` con un dashboard de terminal en vivo para muestreo de registros.
- Comando `serve` con un simulador de esclavo Modbus TCP para pruebas.
- Soporte de escritura en el modo `read` usando `--value` para registros holding y coils.
- Soporte de configuración YAML para modos de conexión TCP y RTU.
- Definición de comandos CLI con `clap`, incluyendo banderas cortas y valores predeterminados.
- Mejoras en el manejo de errores con integración de `anyhow` y `thiserror`.

### Cambiado

- Documentación y ejemplos actualizados para uso en Linux y Windows.
- Soporte Modbus asíncrono reforzado usando `tokio`, `tokio-modbus` y `tokio-serial`.
- Comportamiento del simulador mejorado para soportar códigos de función Modbus de lectura y escritura.

### Corregido

- Se corrigió el simulador Modbus y la lógica de manejo de registros para el modo TCP.
- Se corrigió el parseo de la CLI para los argumentos de tipo de registro, intervalo y endpoint.

## [0.1.0] - 2026-08-01

### Añadido

- Lanzamiento inicial de `modoc` con una estructura básica de CLI para Modbus.
- Soporte para lectura de registros Modbus holding, input, coils y discrete inputs.
- Soporte básico de archivo de configuración YAML para conexiones TCP.
