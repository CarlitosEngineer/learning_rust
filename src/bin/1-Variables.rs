fn learning_variables1() {
    let variable_1: i32 = 10; // Nombre de la variable "variable_1", Tipo de la variable i32. (DATO EXPLICITO, NOT INFERIDO)
    let variable_2 = 100; // Nombre de la variable "variable_2", Tipo de la variable i32. (DATO IMPLICITO, INFERIDO)
    println!("La variable 1 vale: {}", variable_1);
    println!("La variable 2 vale: {}", variable_2);
}

fn main() {
    learning_variables1();
}

// ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // NOTES ====>

/*

¿Qué son las variables?

* La variable es un estructura que se encarga de almacenar información de forma temporal en la memoria.
* las variables estan formadas de tres partes.
  + El identificador de la variable, que es el nombre de la variable.
  + La clase de valores que puede almacenar la variable, que es el tipo de variable.
  + El dato que almacena la variable, que es el valor de la variable.
* las variables se declaran con ´let´, ´let mut´, ´const´.
* En Rust, la declaración de variables se denomina “binding” (vinculación).

📌 Run this code using: cargo run --bin 1-Variables

Fuente consultada:

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

*/
