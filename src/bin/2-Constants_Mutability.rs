/*
¿Que es la Mutabilidad?

R/
- La mutabilidad es la capacidad que tiene una variable de cambiar su valor durante la ejecutacion del programa.
- En Rust, las variables son inmutables por defecto; para hacerlas mutables se usa `mut`.

Usa este comando para ejecutar este código: cargo run --bin 2-Constants_Mutability

Fuente consultada:
- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
*/

const VARIABLE_CONST1: u32 = 100; // Como declarar una constante

fn learning_mutability1() {
    let variable_1 = 0; // No Mutable
    let mut variable_2 = 0; // Mutable

    // variable_1 += 1; // NOT OK
    variable_2 += 1; // OK

    println!("El valor 3 es: {}", variable_1);
    println!("El valor 3 es: {}", variable_2);
    println!("El valor 3 es: {}", VARIABLE_CONST1);
}

fn main() {
    learning_mutability1();
}
