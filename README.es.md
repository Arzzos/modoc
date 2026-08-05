<div align="center">

# modoc

**Una CLI profesional para leer, monitorear y simular dispositivos Modbus TCP/RTU — directo desde tu terminal.**

[![Crates.io](https://img.shields.io/crates/v/modoc.svg)](https://crates.io/crates/modoc)
[![Docs.rs](https://img.shields.io/docsrs/modoc)](https://docs.rs/modoc)
[![License](https://img.shields.io/crates/l/modoc.svg)](#licencia)
[![Downloads](https://img.shields.io/crates/d/modoc.svg)](https://crates.io/crates/modoc)

[**English**](README.md) | **Español**

</div>

---

`modoc` es una herramienta de línea de comandos para trabajar con dispositivos [Modbus](https://es.wikipedia.org/wiki/Modbus) por **TCP** o **RTU (serie)**. Proporciona acceso de lectura/escritura a registros, un dashboard en vivo y un simulador Modbus TCP integrado para pruebas sin hardware.

Versión actual: `0.2.0`

## Características

- **Lectura y escritura** de holding registers, input registers, coils y discrete inputs
- **Soporte para Modbus TCP y RTU** mediante configuración YAML
- **Dashboard en vivo** — interfaz de terminal construida con [ratatui](https://ratatui.rs) y [crossterm](https://crates.io/crates/crossterm)
- **Simulador TCP integrado** con soporte de lectura y escritura
- **CLI definida con `clap`** para parseo estable y salida de `--help` consistente
- **E/S asíncrona** usando [tokio](https://tokio.rs) y [tokio-modbus](https://github.com/slowtec/tokio-modbus)
- Mensajes de error claros para fallos de conexión, configuración y protocolo

## Instalación

### Desde crates.io

```bash
cargo install modoc
```

### Desde GitHub releases

Descarga la versión más reciente en [https://github.com/Arzzos/modoc/releases](https://github.com/Arzzos/modoc/releases).

### Desde el código fuente

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build --release
./target/release/modoc --help
```

> **Requisitos:** Rust 1.75+ (stable). En Linux, el soporte serial (RTU) requiere los headers de `libudev`: `sudo apt install libudev-dev` en Debian/Ubuntu.

## Inicio rápido

1. Crea un archivo de configuración. Consulta [Configuración](#configuración) para ejemplos de TCP y RTU.
2. Lee registros.
3. Escribe un valor en un registro.
4. Monitorea un registro en vivo.
5. Ejecuta el simulador TCP.

### Ejemplo en Linux

```bash
modoc read --register-type holding --address 0 --count 4 --config config.yaml
modoc read --register-type coil --address 10 --value 1 --config config.yaml
modoc monitor --address 0 --interval 500 --config config.yaml
modoc serve --endpoint 502 --mode tcp
```

### Ejemplo en Windows (PowerShell)

```powershell
modoc.exe read --register-type holding --address 0 --count 4 --config config.yaml
modoc.exe read --register-type coil --address 10 --value 1 --config config.yaml
modoc.exe monitor --address 0 --interval 500 --config config.yaml
modoc.exe serve --endpoint 502 --mode tcp
```

## Configuración

`modoc` lee un archivo YAML que describe cómo conectarse a tu dispositivo. Pasa la ruta con `--config` (por defecto `config.yaml`).

**TCP:**

```yaml
connection:
  mode: tcp
  host: 192.168.1.50
  port: 502
```

**RTU (serie):**

```yaml
connection:
  mode: rtu
  serial_port: /dev/ttyUSB0   # o COM3 en Windows
  baud_rate: 9600
  data_bits: 8
  stop_bits: 1
  parity: none                # none | odd | even
```

## Uso

```bash
modoc <COMMAND>
```

### Comandos

- `read` — Lee o escribe registros Modbus (holding, input, coil, discrete)
- `monitor` — Inicia un dashboard de terminal en vivo para un registro
- `serve` — Ejecuta un esclavo virtual Modbus para pruebas
- `help` — Muestra este mensaje o la ayuda del subcomando dado

### `read`

| Flag | Descripción | Default |
|---|---|---|
| `-t, --register-type` | Tipo de registro: `holding`, `input`, `coil` o `discrete` | `holding` |
| `-a, --address` | Dirección inicial del registro | — |
| `-n, --count` | Número de registros a leer (ignorado si se escribe) | `1` |
| `-v, --value` | Valor a escribir; habilita el modo escritura | — |
| `-c, --config` | Ruta del archivo de configuración YAML | `config.yaml` |

### `monitor`

| Flag | Descripción | Default |
|---|---|---|
| `-a, --address` | Dirección del registro a monitorear | — |
| `-i, --interval` | Intervalo de muestreo en milisegundos | `500` |
| `-c, --config` | Ruta del archivo de configuración YAML | `config.yaml` |

### `serve`

| Flag | Descripción | Default |
|---|---|---|
| `-e, --endpoint` | Puerto TCP (ej. `502`) o nombre de puerto serie para RTU | `502` |
| `-m, --mode` | `tcp` o `rtu` | `tcp` |

> Nota: `serve --mode tcp` inicia el simulador Modbus TCP integrado. `serve --mode rtu` se acepta en la CLI, pero el soporte RTU aún no está implementado.

Ejecuta `modoc <comando> --help` para ver la referencia de CLI actual.

## Cómo está construido

```
src/
├── cli.rs        # Definición de la interfaz de línea de comandos (clap)
├── core/         # Lógica de dominio: carga de config, operaciones de lectura/escritura, errores
├── protocol/     # Manejo de clientes Modbus TCP y RTU (tokio-modbus)
├── ui/           # Dashboard de terminal (ratatui + crossterm)
└── simulator/    # Simulador de esclavo Modbus TCP integrado
```

## Desarrollo

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
```

### Dependencias recomendadas en Linux

```bash
sudo apt update
sudo apt install libudev-dev
```

### Notas para Windows

Instala Rust con [rustup](https://rustup.rs/) y utiliza puertos `COM` para dispositivos RTU. Ejecuta PowerShell como Administrador si necesitas acceso al hardware serial.

Las contribuciones, reportes de bugs y solicitudes de funcionalidades son bienvenidas — abre un [issue](https://github.com/Arzzos/modoc/issues) o un pull request.

## Roadmap

- [x] Comando `read` básico para registros holding, input, coil y discrete
- [x] Modo escritura para valores de holding y coil con `read --value`
- [x] Comando `monitor` con dashboard de terminal en vivo
- [x] Comando `serve` con simulación Modbus TCP y soporte de lectura/escritura
- [ ] Simulación RTU en `serve`
- [ ] Soporte de escritura para múltiples registros/coils en una sola llamada
- [ ] Exportar el historial del monitor a CSV
- [ ] Validación de configuración con diagnósticos más claros

## Licencia

Licenciado bajo cualquiera de las siguientes opciones:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

a tu elección.

## Agradecimientos

Construido sobre los excelentes crates [tokio-modbus](https://github.com/slowtec/tokio-modbus), [tokio-serial](https://github.com/berkowski/tokio-serial), [ratatui](https://ratatui.rs) y [clap](https://github.com/clap-rs/clap) crates.