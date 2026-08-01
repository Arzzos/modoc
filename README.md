<div align="center">

# modoc

**A professional CLI to read, monitor, and simulate Modbus TCP/RTU devices — right from your terminal.**

[![Crates.io](https://img.shields.io/crates/v/modoc.svg)](https://crates.io/crates/modoc)
[![Docs.rs](https://img.shields.io/docsrs/modoc)](https://docs.rs/modoc)
[![License](https://img.shields.io/crates/l/modoc.svg)](#license)
[![Downloads](https://img.shields.io/crates/d/modoc.svg)](https://crates.io/crates/modoc)

**English** | [**Español**](README.es.md)

</div>

---

`modoc` is a command-line tool for working with [Modbus](https://en.wikipedia.org/wiki/Modbus) devices over **TCP** or **RTU (serial)**. It's built for engineers and technicians who need to read/write registers, watch a value change live, or spin up a fake Modbus slave to test other software — without opening a GUI.

```
$ modoc read --register-type holding --address 0 --count 4 --config config.yaml
Registros leídos (tipo: holding):
  [0] = 512
  [1] = 128
  [2] = 0
  [3] = 7
```

## Features

- **Read & write** holding registers, input registers, coils, and discrete inputs
- **Modbus TCP and RTU** support, configured via a single YAML file
- **Live dashboard** — a terminal UI (built with [ratatui](https://ratatui.rs)) that plots a register's value over time as a sparkline
- **Built-in slave simulator** — spin up a fake Modbus TCP device to test integrations without real hardware
- **Async under the hood** — built on [tokio](https://tokio.rs) and [tokio-modbus](https://github.com/slowtec/tokio-modbus)
- Clear, actionable error messages for connection, configuration, and protocol errors

## Installation

### From crates.io

```bash
cargo install modoc
```

### From source

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build --release
./target/release/modoc --help
```

> **Requirements:** Rust 1.75+ (stable). On Linux, reading serial (RTU) ports also requires `libudev` development headers (`sudo apt install libudev-dev` on Debian/Ubuntu).

## Quick Start

1. **Create a config file.** See [Configuration](#configuration) below for TCP and RTU examples.

2. **Read some registers:**

   ```bash
   modoc read --register-type holding --address 0 --count 4 --config config.yaml
   ```

3. **Write a value:**

   ```bash
   modoc read --register-type holding --address 0 --value 42 --config config.yaml
   ```

4. **Watch a register live:**

   ```bash
   modoc monitor --address 0 --interval 500 --config config.yaml
   ```

   Press `q` or `Esc` to exit the dashboard.

5. **Simulate a device** (useful for testing without real hardware):

   ```bash
   modoc serve --endpoint 502 --mode tcp
   ```

## Configuration

`modoc` reads a YAML file describing how to connect to your device. Pass its path with `--config` (defaults to `config.yaml` in the current directory).

**TCP:**

```yaml
connection:
  mode: tcp
  host: 192.168.1.50
  port: 502
```

**RTU (serial):**

```yaml
connection:
  mode: rtu
  serial_port: /dev/ttyUSB0   # or COM3 on Windows
  baud_rate: 9600
  data_bits: 8
  stop_bits: 1
  parity: none                # none | odd | even
```

## Usage

```
modoc <COMMAND>

Commands:
  read     Read or write Modbus registers (holding, input, coil, discrete)
  monitor  Launch a live terminal dashboard for a register
  serve    Run a virtual Modbus slave for testing
  help     Print this message or the help of the given subcommand
```

### `read`

| Flag | Description | Default |
|---|---|---|
| `--register-type` | `holding`, `input`, `coil`, or `discrete` | `holding` |
| `--address` | Starting register address | — |
| `--count` | Number of registers to read (ignored when writing) | `1` |
| `--value` | Value to write. If set, performs a write instead of a read | — |
| `--config` | Path to the YAML config file | `config.yaml` |

### `monitor`

| Flag | Description | Default |
|---|---|---|
| `--address` | Register address to monitor | — |
| `--interval` | Polling interval in milliseconds | `500` |
| `--config` | Path to the YAML config file | `config.yaml` |

### `serve`

| Flag | Description | Default |
|---|---|---|
| `--endpoint` | TCP port (e.g. `502`) or serial port name for RTU | `502` |
| `--mode` | `tcp` or `rtu` (RTU simulation is currently a stub) | `tcp` |

Run `modoc <command> --help` for the full, always-up-to-date list of flags.

## How it's built

```
src/
├── cli.rs        # Command-line interface definition (clap)
├── core/         # Domain logic: config loading, register read/write ops, errors
├── protocol/     # Modbus TCP and RTU client connection handling (tokio-modbus)
├── ui/           # Terminal dashboard (ratatui + crossterm)
└── simulator/    # Built-in Modbus TCP slave for testing
```

## Development

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build
cargo test
cargo clippy --all-targets --all-features
```

Contributions, bug reports, and feature requests are welcome — please open an [issue](https://github.com/Arzzos/modoc/issues) or a pull request.

## Roadmap

- [ ] RTU simulation in `serve`
- [ ] Write support for multiple registers / coils in a single call
- [ ] Export monitor history to CSV
- [ ] Config validation with clearer diagnostics

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Acknowledgments

Built on top of the excellent [tokio-modbus](https://github.com/slowtec/tokio-modbus), [tokio-serial](https://github.com/berkowski/tokio-serial), [ratatui](https://ratatui.rs), and [clap](https://github.com/clap-rs/clap) crates.