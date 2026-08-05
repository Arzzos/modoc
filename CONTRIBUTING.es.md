# Contribuyendo a modoc

Gracias por tu interés en contribuir a `modoc`.

Esta guía explica cómo clonar el repositorio, preparar el entorno de desarrollo en Linux y Windows, ejecutar pruebas, formatear el código y enviar un pull request.

## Primeros pasos

```bash
git clone https://github.com/Arzzos/modoc.git
cd modoc
```

## Requisitos previos

- Rust 1.75 o superior
- Herramientas de `cargo` instaladas con [rustup](https://rustup.rs/)

### Linux

Para soporte serial RTU, instala los headers de desarrollo de `libudev`:

```bash
sudo apt update
sudo apt install libudev-dev
```

Si tu distribución requiere herramientas de compilación adicionales, instálalas según sea necesario.

### Windows

- Instala Rust con [rustup](https://rustup.rs/)
- Usa puertos `COM` para conexiones serial/RTU
- Ejecuta PowerShell o Command Prompt como Administrador si necesitas acceso al hardware

## Compilar y ejecutar

```bash
cargo build
cargo run -- --help
```

## Pruebas

Ejecuta la suite de pruebas del repositorio:

```bash
cargo test
```

## Formateo

Formatea el código fuente con:

```bash
cargo fmt --all
```

## Linting

Ejecuta Clippy para detectar problemas comunes:

```bash
cargo clippy --all-targets --all-features
```

## Trabajando en un cambio

1. Crea una rama de trabajo:

```bash
git checkout -b feature/tu-mejora
```

2. Realiza tus cambios.
3. Ejecuta `cargo test`, `cargo fmt --all` y `cargo clippy --all-targets --all-features`.
4. Haz commit con un mensaje claro.

## Flujo de pull request

1. Empuja tu rama:

```bash
git push origin feature/tu-mejora
```

2. Abre un pull request en GitHub.
3. Describe el cambio y cómo probarlo.
4. Enlaza issues relacionados si aplica.

## Notas

- Mantén los cambios enfocados y pequeños.
- Actualiza la documentación cuando cambie el comportamiento.
- Si agregas nuevas opciones de CLI, incluye texto de ayuda y ejemplos.
- El CI debe pasar antes de fusionar.

Gracias por ayudar a mejorar `modoc`.
