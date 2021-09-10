# Analizador Léxico

## Requisitos previos

1. [rustp](https://www.rust-lang.org/tools/install): Este programa está escrito en `rust`, por lo tanto necesita . Si usas `macOS`, `Linux` u otro sistema derivado de `Unix`, puedes instalarlo con:
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

3. Puedes ejecutar el programa directamente con `cargo` (se instala automáticamente con `rustp`):
  ```bash
  # el programa necesita un archivo como entrada para correr correctamente
  cargo run < src/input-examples.txt
  ```
