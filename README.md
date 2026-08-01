# Modoc - Industrial Modbus CLI & TUI Tool  
  
[!\[Crates.io\](https://img.shields.io/crates/v/modoc)\](https://crates.io/crates/modoc)  
[!\[License\](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)\](LICENSE)  
[!\[Build Status\](https://github.com/Arzzos/modoc/workflows/CI/badge.svg)\](https://github.com/Arzzos/modoc/actions)  
  
---  
  
## English Description  
  
Modoc is a modern, ultra-fast, cross-platform CLI and TUI companion for Modbus TCP/RTU protocol inspection, real-time data streaming monitoring, and hardware simulation built in Rust.  
  
### Features  
- \*\*Intuitive Commands\*\*: Built-in support for \`read\`, \`monitor\`, and \`serve\` context operations.  
- \*\*Terminal User Interface (TUI)\*\*: Live analytical dashboard utilizing interactive sparklines and widgets powered by \`ratatui\`.  
- \*\*Dual Support\*\*: Hardware Abstraction Layer for both TCP (Ethernet) and RTU (RS485/Serial port) environments.  
- \*\*Industrial Resilience\*\*: Asynchronous, non-blocking network core driven by \`tokio\` featuring robust CRC, timeout, and exception error handling.  
- \*\*Built-in Simulator\*\*: Virtual Modbus server orchestration to mock hardware behaviors for local environment testing.  
  
---  
  
## Descripción en Español  
  
Modoc es una herramienta de línea de comandos moderna y eficiente para interactuar con dispositivos Modbus TCP/RTU. Permite leer y escribir registros, monitorizar variables en tiempo real con una interfaz TUI gráfica, y simular un entorno esclavo Modbus para pruebas locales sin hardware real.  
  
### Características  
- \*\*Comandos intuitivos\*\*: Operaciones nativas mediante \`read\`, \`monitor\` y \`serve\`.  
- \*\*Interfaz de usuario en terminal (TUI)\*\*: Paneles dinámicos con gráficas sparkline y componentes visuales avanzados.  
- \*\*Soporte dual\*\*: Conectividad nativa para arquitecturas TCP (Ethernet) y RTU (RS485/puertos serie).  
- \*\*Manejo robusto de errores\*\*: Detección y reporte detallado de fallos industriales críticos (errores de CRC, timeouts y excepciones Modbus).  
- \*\*Asíncrono y no bloqueante\*\*: Núcleo de alto rendimiento basado en el ecosistema \`tokio\`.  
- \*\*Simulador integrado\*\*: Motor virtual esclavo para emulación de dispositivos en entornos de desarrollo.  
- \*\*Configuración flexible\*\*: Soporte centralizado para mapeo mediante archivos de configuración YAML.  
  
---  
  
## Installation / Instalación  
  
### From / Desde Crates.io  
```bash  
cargo install modoc  
```

### From Source / Desde Fuente

```bash

git clone \[https://github.com/Arzzos/modoc.git\](https://github.com/Arzzos/modoc.git)  
cd modoc  
cargo build --release  
```

## Basic Usage / Uso Básico

### Read holding registers / Leer registros holding

```Bash

modoc read --address 0 --count 10 --register-type holding  
```

### Write a value to a holding register / Escribir un valor en un registro holding

```Bash

modoc write --address 5 --value 1234  
```

### Real-time streaming monitoring / Monitoreo en tiempo real

```Bash

modoc monitor --address 100 --interval 200  
```

### Simulate a virtual TCP slave / Simular un esclavo Modbus TCP

```Bash

modoc serve --endpoint 502 --mode tcp  
```

## Configuration / Configuración (`config.yaml`)

### TCP Mode Setup

```YAML

connection:  
  mode: tcp  
  host: 192.168.1.100  
  port: 502  
```

### RTU (Serial/RS485) Mode Setup

```YAML

connection:  
  mode: rtu  
  serial\_port: /dev/ttyUSB0  
  baud\_rate: 9600  
  data\_bits: 8  
  stop\_bits: 1  
  parity: None  
```

## Contributing / Cómo Contribuir

¡Gracias por tu interés en colaborar en Modoc! Para mantener el repositorio limpio y profesional bajo las normas de código abierto, seguimos este flujo de trabajo estricto:

### Workflow / Flujo de Desarrollo

1.  ****Issues First****: Antes de escribir código, busca o abre un ****Issue**** en GitHub para discutir la característica o el fallo. Asínatelo a ti mismo para evitar conflictos de trabajo simultáneo.
2.  ****Feature Branches****: No trabajes sobre `main`. Crea siempre una rama dedicada para tu tarea:  
    Bash
    
    git checkout -b feat/issue-id-short-description  
    
3.  ****Quality Gates****: Asegúrate de que el analizador estático y las pruebas unitarias pasen sin ninguna advertencia antes de enviar tu código:  
    Bash
    
    cargo fmt --check  
    cargo clippy -- -D warnings  
    cargo test  
    
4.  ****Pull Requests****: Abre un PR apuntando a la rama principal e incluye la etiqueta `Closes #numero_de_issue` en la descripción para la automatización del repositorio.

### Commits Standards

Seguimos estrictamente la convención de ****Conventional Commits****. Los mensajes deben estructurarse imperativamente en inglés y en presente, por ejemplo:

-   `feat: add RTU serial connection layer`
-   `fix: resolve memory leak in TUI loop context`
-   `chore: update workflow cache configurations`

## License / Licencia

Dual-licensed under either ****MIT License**** or ****Apache License, Version 2.0****. Este proyecto está licenciado bajo los términos compartidos de la ****Licencia MIT**** o ****Apache-2.0****.