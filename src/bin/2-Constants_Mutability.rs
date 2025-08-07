fn learning_mutability1() {
    const VARIABLE_CONST1: u32 = 100; // DECLARAMOS UNA CONSTANTE
    let variable_1 = 0; // NO MUTABLE
    let mut variable_2 = 0; // MUTABLE

    // variable_1 += 1; // NOT OK
    variable_2 += 1; // OK

    println!("LA VARIABLE 1 NOT MUTABLE: {}", variable_1);
    println!("LA VARIABLE 2 MUTABLE: {}", variable_2);
    println!("LA VARIABLE 3 CONSTANTE: {}", VARIABLE_CONST1);
}

fn main() {
    learning_mutability1();
}

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // NOTES ====>

/*
¿Que es la Mutabilidad?

R/

* La mutabilidad es la capacidad que tiene una variable de cambiar su valor durante la ejecutacion del programa.
* En Rust, las variables son inmutables por defecto; para hacerlas mutables se usa `mut`.

Usa este comando para ejecutar este código: cargo run --bin 2-Constants_Mutability

Fuente consultada:

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

*/
