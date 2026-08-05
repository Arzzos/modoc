# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-08-04

### Added

- `monitor` command with a live terminal dashboard for register sampling.
- `serve` command with a Modbus TCP slave simulator for testing.
- Write support in `read` mode using `--value` for holding registers and coils.
- YAML configuration support for both TCP and RTU connection modes.
- CLI command definitions with `clap`, including short flags and default values.
- Error handling improvements with clearer `anyhow` and `thiserror` integration.

### Changed

- Updated documentation and examples for Linux and Windows usage.
- Strengthened async Modbus support using `tokio`, `tokio-modbus`, and `tokio-serial`.
- Improved simulator behavior to support read and write Modbus function codes.

### Fixed

- Fixed Modbus simulator and register handling logic for TCP mode.
- Fixed CLI parsing for register type, interval, and endpoint arguments.

## [0.1.0] - 2026-08-01

### Added

- Initial release of `modoc` with a basic Modbus CLI structure.
- Support for reading Modbus holding registers, input registers, coils, and discrete inputs.
- Basic YAML configuration file support for TCP connections.
