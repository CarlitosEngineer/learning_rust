fn learning_mutability1() {
    const VARIABLE_CONST1: u32 = 100; // [ NOT MUTABLE , UNSIGNED]
    let variable_1 = -2147483648; // [ NOT MUTABLE , INFERIDO(INPLICITO), 32BITS POR DEFECTO]
    let mut variable_2: i64 = -9223372036854775808; // [ MUTABLE, NOT INFERIDO(EXPLICITO) ]

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

📌 Run this code using: cargo run --bin 2-Constants_Mutability

Fuente consultada:

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

*/
