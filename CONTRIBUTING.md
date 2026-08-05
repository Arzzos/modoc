# Contributing to modoc

Thank you for your interest in contributing to `modoc`.

This guide explains how to clone the repository, prepare your development environment on Linux and Windows, run tests, format code, and submit a pull request.

## Getting started

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
```

## Prerequisites

- Rust 1.75 or newer
- `cargo` toolchain installed via [rustup](https://rustup.rs/)

### Linux

For RTU serial support, install the `libudev` development headers:

```bash
sudo apt update
sudo apt install libudev-dev
```

If your distribution requires additional build tools, install them as needed.

### Windows

- Install Rust with [rustup](https://rustup.rs/)
- Use `COM` ports for serial/RTU connections
- Run PowerShell or Command Prompt as Administrator if you need hardware access

## Build and run

```bash
cargo build
cargo run -- --help
```

## Testing

Run the repository test suite:

```bash
cargo test
```

## Formatting

Format source files with:

```bash
cargo fmt --all
```

## Linting

Run Clippy to catch common issues:

```bash
cargo clippy --all-targets --all-features
```

## Working on a change

1. Create a topic branch:

```bash
git checkout -b feature/your-improvement
```

2. Make your changes.
3. Run `cargo test`, `cargo fmt --all`, and `cargo clippy --all-targets --all-features`.
4. Commit your work with a clear message.

## Pull request workflow

1. Push your branch:

```bash
git push origin feature/your-improvement
```

2. Open a pull request on GitHub.
3. Describe the change, why it is needed, and how to test it.
4. Link related issues if applicable.

## Notes

- Keep changes focused and small.
- Update documentation when behavior changes.
- If you add new CLI options, include help text and examples.
- CI should pass before merging.

Thank you for helping improve `modoc`.
