
# 🚀 **Command Lists**

| **Comando**           | **Descripción** |
|----------------------|----------------|
| `cargo new <nombre>` | Crea un nuevo proyecto |
| `cargo run`          | Compila y ejecuta el código |
| `cargo build`        | Compila el código sin ejecutarlo |
| `cargo check`        | Verifica errores sin compilar |
| `cargo add <dep>`    | Agrega una dependencia |
| `cargo update`       | Actualiza las dependencias |
| `cargo test`         | Ejecuta pruebas |
| `cargo doc --open`   | Genera documentación |
| `cargo clean`        | Limpia archivos de compilación |

- **[Commands List](https://doc.rust-lang.org/cargo/commands/index.html)** - Lista de comandos que pueden servirte.

## **Comandos de Cargo (Gestor de Proyectos en Rust)**

```sh
cargo new <proyecto>       # Crear un nuevo proyecto de Rust
cargo build                # Compilar el proyecto
cargo run                  # Compilar y ejecutar el proyecto
cargo check                # Verificar errores sin compilar
cargo clean                # Eliminar archivos de compilación
cargo update               # Actualizar dependencias en Cargo.toml
cargo add <dependencia>    # Agregar una dependencia al proyecto
cargo remove <dependencia> # Eliminar una dependencia
cargo doc --open           # Generar y ver documentación del proyecto
```

## **Comandos de `rustc` (Compilador de Rust)**

```sh
rustc --version        # Ver versión de Rust instalada
rustc main.rs          # Compilar un archivo Rust manualmente
./main                 # Ejecutar el binario compilado
```

## **Comandos de `rustup` (Gestor de Rust)**

```sh
rustup show                  # Ver la versión actual de Rust y el toolchain usado
rustup update                # Actualizar Rust a la última versión
rustup default stable         # Configurar la versión estable por defecto
rustup install nightly        # Instalar la versión nightly de Rust
rustup target list            # Listar plataformas de compilación disponibles
rustup component add clippy   # Instalar Clippy (linter para Rust)
rustup component add rustfmt  # Instalar Rustfmt (formateador de código)
```

## **Comandos de `rustfmt` (Formateador de código)**

```sh
rustfmt main.rs       # Formatear un archivo Rust
cargo fmt             # Formatear todo el proyecto
```

## **Comandos de `clippy` (Linter para Rust)**

```sh
cargo clippy         # Revisar el código en busca de mejoras y optimización
```
