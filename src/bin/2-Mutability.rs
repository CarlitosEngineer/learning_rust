/*
¿Que es la Mutabilidad?

R/
- La mutabilidad es la capacidad que tiene una variable de cambiar sus datos.
- En Rust, las variables son inmutables por defecto; para hacerlas mutables se usa `mut`.

Fuente consultada:
- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
*/

fn learning_mutability1() {
    let variable_1 = 0;
    let mut variable_2 = 0;

    // variable_1 += 1; // NOT OK
    variable_2 += 1; // OK

    println!("El valor 3 es: {}", variable_1);
    println!("El valor 3 es: {}", variable_2);
}

fn main() {
    learning_mutability1();
}
