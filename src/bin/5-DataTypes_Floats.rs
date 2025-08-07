fn testing_datatypes_floats() {
    let variable_01: f32 = 3.1415927; // Ejemplo f32 positivo
    let variable_02: f32 = -2.7182818; // Ejemplo f32 negativo
    let variable_03: f64 = 3.141592653589793; // Ejemplo f64 positivo
    let variable_04: f64 = -2.718281828459045; // Ejemplo f64 negativo
    println!("f32 positivo: {}", variable_01);
    println!("f32 negativo: {}", variable_02);
    println!("f64 positivo: {}", variable_03);
    println!("f64 negativo: {}", variable_04);
    println!("f32 MIN_POSITIVE: {}", f32::MIN_POSITIVE); // IMPRIME LOS VALORES MAS PEQUELOS POSIBLES 32BITS
    println!("f64 MIN_POSITIVE: {}", f64::MIN_POSITIVE); // IMPRIME LOS VALORES MAS PEQUELOS POSIBLES 64BITS
    println!("f32 MAX: {}", f32::MAX); // ~3.40282347e+38
    println!("f64 MAX: {}", f64::MAX); // ~1.7976931348623157e+308
    println!("f32 MIN: {}", f32::MIN); // ~-3.40282347e+38
    println!("f64 MIN: {}", f64::MIN); // ~-1.7976931348623157e+308
    println!("------ VALORES NORMALES ------");
    println!("f32 positivo: {}", variable_01);
}

fn main() {
    testing_datatypes_floats();
}

/*

* Un float es un número con decimales (punto flotante), usado para representar valores reales.

| Tipo  | Bits | Precisión       | Rango aprox.                    |
| ----- | ---- | --------------- | ------------------------------- |
| `f32` | 32   | \~7 dígitos     | ±1.175494e-38 a ±3.402823e+38   |
| `f64` | 64   | \~15-17 dígitos | ±2.225073e-308 a ±1.797693e+308 |

Usa este comando para ejecutar este código: cargo run --bin 5-DataTypes_Floats

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
    https://doc.rust-lang.org/book/ch03-02-data-types.html

* ¡Cuándo NO usar float! No uses float cuando necesitas exactitud total en decimales, por ejemplo: Dinero 💰, Cálculos bancarios, Contabilidad
* SE RECOMIENDA USAR LIBRERIA → En esos casos, usa tipos como Decimal del crate rust_decimal.
    // https://docs.rs/rust_decimal/latest/rust_decimal/

*/
