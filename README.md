# Analizador Léxico

Herramienta escrita en Rust para analizar gramáticas en la forma normal de Chompsky.

## Requisitos previos

1. [rustup](https://www.rust-lang.org/tools/install): Este programa está escrito en `rust`, por lo tanto se necesitan las herramientas de desarrollo de `rust`. Si usas `macOS`, `Linux` u otro sistema derivado de `Unix`, puedes instalarlo con:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Instalando

1. Primero, descarga el repo:
  ```bash
  git clone git@github.com:Topi99/lexian.git
  ```

2. Entra al nuevo directorio:
  ```bash
  cd lexian
  ```

3. Puedes ejecutar el programa directamente con `cargo` (se instala automáticamente con `rustup`), pero se recomienda ejecutar en modo "release":
  ```bash
  # el programa necesita un archivo como entrada para correr correctamente
  cargo run < src/examples/input2.txt
  ```

4. Para ejecutar en modo "release", primero se debe construir el programa:
  ```bash
  cargo build --release
  ```
  El binario se encuentra en `./target/release/lexian`. Para ejecutar:
  ```bash
  # el programa necesita un archivo como entrada para correr correctamente
  ./target/release/lexian < ./src/examples/input2.txt
  ```
