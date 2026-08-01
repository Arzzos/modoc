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

`modoc` es una herramienta de línea de comandos para trabajar con dispositivos [Modbus](https://es.wikipedia.org/wiki/Modbus) por **TCP** o **RTU (serie)**. Está pensada para ingenieros y técnicos que necesitan leer/escribir registros, ver un valor cambiar en tiempo real, o levantar un esclavo Modbus falso para probar otro software — sin necesidad de abrir una GUI.

```
$ modoc read --register-type holding --address 0 --count 4 --config config.yaml
Registros leídos (tipo: holding):
  [0] = 512
  [1] = 128
  [2] = 0
  [3] = 7
```

## Características

- **Lectura y escritura** de holding registers, input registers, coils y discrete inputs
- **Soporte para Modbus TCP y RTU**, configurado con un único archivo YAML
- **Dashboard en vivo** — una interfaz de terminal (construida con [ratatui](https://ratatui.rs)) que grafica el valor de un registro en el tiempo como un sparkline
- **Simulador de esclavo integrado** — levanta un dispositivo Modbus TCP falso para probar integraciones sin hardware real
- **Asíncrono por dentro** — construido sobre [tokio](https://tokio.rs) y [tokio-modbus](https://github.com/slowtec/tokio-modbus)
- Mensajes de error claros y accionables para problemas de conexión, configuración y protocolo

## Instalación

### Desde crates.io

```bash
cargo install modoc
```

### Desde el código fuente

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build --release
./target/release/modoc --help
```

> **Requisitos:** Rust 1.75+ (stable). En Linux, leer puertos serie (RTU) también requiere los headers de desarrollo de `libudev` (`sudo apt install libudev-dev` en Debian/Ubuntu).

## Inicio rápido

1. **Crea un archivo de configuración.** Revisa la sección [Configuración](#configuración) más abajo para ejemplos de TCP y RTU.

2. **Lee algunos registros:**

   ```bash
   modoc read --register-type holding --address 0 --count 4 --config config.yaml
   ```

3. **Escribe un valor:**

   ```bash
   modoc read --register-type holding --address 0 --value 42 --config config.yaml
   ```

4. **Observa un registro en vivo:**

   ```bash
   modoc monitor --address 0 --interval 500 --config config.yaml
   ```

   Presiona `q` o `Esc` para salir del dashboard.

5. **Simula un dispositivo** (útil para probar sin hardware real):

   ```bash
   modoc serve --endpoint 502 --mode tcp
   ```

## Configuración

`modoc` lee un archivo YAML que describe cómo conectarse a tu dispositivo. Pásale la ruta con `--config` (por defecto usa `config.yaml` en el directorio actual).

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

```
modoc <COMMAND>

Commands:
  read     Lee o escribe registros Modbus (holding, input, coil, discrete)
  monitor  Inicia un dashboard de terminal en vivo para un registro
  serve    Ejecuta un esclavo Modbus virtual para pruebas
  help     Muestra este mensaje o la ayuda de un subcomando
```

### `read`

| Flag | Descripción | Default |
|---|---|---|
| `--register-type` | `holding`, `input`, `coil`, o `discrete` | `holding` |
| `--address` | Dirección de inicio del registro | — |
| `--count` | Cantidad de registros a leer (se ignora si se escribe) | `1` |
| `--value` | Valor a escribir. Si se indica, se realiza una escritura en lugar de una lectura | — |
| `--config` | Ruta al archivo de configuración YAML | `config.yaml` |

### `monitor`

| Flag | Descripción | Default |
|---|---|---|
| `--address` | Dirección del registro a monitorear | — |
| `--interval` | Intervalo de muestreo en milisegundos | `500` |
| `--config` | Ruta al archivo de configuración YAML | `config.yaml` |

### `serve`

| Flag | Descripción | Default |
|---|---|---|
| `--endpoint` | Puerto TCP (ej. `502`) o nombre del puerto serie para RTU | `502` |
| `--mode` | `tcp` o `rtu` (la simulación RTU aún está pendiente) | `tcp` |

Ejecuta `modoc <comando> --help` para ver la lista completa y siempre actualizada de flags.

## Cómo está construido

```
src/
├── cli.rs        # Definición de la interfaz de línea de comandos (clap)
├── core/         # Lógica de dominio: carga de config, operaciones de lectura/escritura, errores
├── protocol/     # Manejo de conexión de clientes Modbus TCP y RTU (tokio-modbus)
├── ui/           # Dashboard de terminal (ratatui + crossterm)
└── simulator/    # Esclavo Modbus TCP integrado para pruebas
```

## Desarrollo

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build
cargo test
cargo clippy --all-targets --all-features
```

Las contribuciones, reportes de bugs y solicitudes de funcionalidades son bienvenidas — abre un [issue](https://github.com/Arzzos/modoc/issues) o un pull request.

## Roadmap

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

Construido sobre los excelentes crates [tokio-modbus](https://github.com/slowtec/tokio-modbus), [tokio-serial](https://github.com/berkowski/tokio-serial), [ratatui](https://ratatui.rs) y [clap](https://github.com/clap-rs/clap).