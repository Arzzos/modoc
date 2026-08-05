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

`modoc` is a command-line tool for working with [Modbus](https://en.wikipedia.org/wiki/Modbus) devices over **TCP** or **RTU (serial)**. It provides read/write access to registers, a live monitoring dashboard, and a built-in Modbus TCP simulator for testing without hardware.

Current version: `0.2.0`

## Features

- **Read & write** holding registers, input registers, coils, and discrete inputs
- **Modbus TCP and RTU** support via YAML configuration
- **Live dashboard** — terminal UI built with [ratatui](https://ratatui.rs) and [crossterm](https://crates.io/crates/crossterm)
- **Built-in TCP slave simulator** with read and write support
- **CLI defined with `clap`** for stable parsing and consistent `--help` output
- **Async I/O** using [tokio](https://tokio.rs) and [tokio-modbus](https://github.com/slowtec/tokio-modbus)
- Clear error messages for connection, configuration, and protocol issues

## Installation

### From crates.io

```bash
cargo install modoc
```

### From GitHub releases

Download the latest release from [https://github.com/Arzzos/modoc/releases](https://github.com/Arzzos/modoc/releases).

### From source

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build --release
./target/release/modoc --help
```

> **Requirements:** Rust 1.75+ (stable). On Linux, serial (RTU) support requires `libudev-dev` headers: `sudo apt install libudev-dev` on Debian/Ubuntu.

## Quick Start

1. Create a config file. See [Configuration](#configuration) for TCP and RTU examples.
2. Read registers.
3. Write a register value.
4. Monitor a register live.
5. Run the TCP simulator.

### Linux example

```bash
modoc read --register-type holding --address 0 --count 4 --config config.yaml
modoc read --register-type coil --address 10 --value 1 --config config.yaml
modoc monitor --address 0 --interval 500 --config config.yaml
modoc serve --endpoint 502 --mode tcp
```

### Windows example (PowerShell)

```powershell
modoc.exe read --register-type holding --address 0 --count 4 --config config.yaml
modoc.exe read --register-type coil --address 10 --value 1 --config config.yaml
modoc.exe monitor --address 0 --interval 500 --config config.yaml
modoc.exe serve --endpoint 502 --mode tcp
```

## Configuration

`modoc` reads a YAML file describing how to connect to your device. Pass its path with `--config` (defaults to `config.yaml`).

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

```bash
modoc <COMMAND>
```

### Commands

- `read` — Read or write Modbus registers (holding, input, coil, discrete)
- `monitor` — Launch a live terminal dashboard for a register
- `serve` — Run a virtual Modbus slave for testing
- `help` — Print this message or the help of the given subcommand

### `read`

| Flag | Description | Default |
|---|---|---|
| `-t, --register-type` | Register type: `holding`, `input`, `coil`, or `discrete` | `holding` |
| `-a, --address` | Starting register address | — |
| `-n, --count` | Number of registers to read (ignored when writing) | `1` |
| `-v, --value` | Value to write; enables write mode | — |
| `-c, --config` | Path to the YAML configuration file | `config.yaml` |

### `monitor`

| Flag | Description | Default |
|---|---|---|
| `-a, --address` | Register address to monitor | — |
| `-i, --interval` | Polling interval in milliseconds | `500` |
| `-c, --config` | Path to the YAML configuration file | `config.yaml` |

### `serve`

| Flag | Description | Default |
|---|---|---|
| `-e, --endpoint` | TCP port (e.g. `502`) or serial port name for RTU | `502` |
| `-m, --mode` | `tcp` or `rtu` | `tcp` |

> Note: `serve --mode tcp` starts the built-in Modbus TCP simulator. `serve --mode rtu` is accepted by the CLI but RTU simulator support is not implemented yet.

Run `modoc <command> --help` for the current CLI reference.

## How it's built

```
src/
├── cli.rs        # Command-line interface definition (clap)
├── core/         # Domain logic: config loading, register read/write operations, errors
├── protocol/     # Modbus TCP and RTU client handling (tokio-modbus)
├── ui/           # Terminal dashboard (ratatui + crossterm)
└── simulator/    # Built-in Modbus TCP slave simulator
```

## Development

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
cargo build
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
```

### Recommended Linux dependencies

```bash
sudo apt update
sudo apt install libudev-dev
```

### Recommended Windows notes

Install Rust with [rustup](https://rustup.rs/) and use `COM` ports for RTU devices. Run PowerShell as Administrator if you need access to serial hardware.

Contributions, bug reports, and feature requests are welcome — please open an [issue](https://github.com/Arzzos/modoc/issues) or a pull request.

## Roadmap

- [x] Basic `read` command for holding, input, coil, and discrete registers
- [x] Write mode for holding and coil values via `read --value`
- [x] `monitor` command with live terminal dashboard
- [x] `serve` command with Modbus TCP simulation and read/write support
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