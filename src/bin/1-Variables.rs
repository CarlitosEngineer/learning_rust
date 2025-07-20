/*
¿Qué son las variables?

R/
- La variable es un paquete de datos que se encargan de almacenar información de forma temporal.
- las variables se conforman en dos partes, el tipo de variable y el valor de la variable.
- las variables se declaran con ´let´, ´let mut´, ´const´.
- Rust llama a la declaración de variables "Binding (Vinculación)"

Usa este comando para ejecutar el código: cargo run --bin 1-Variables

Fuente consultada:
- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
*/

fn learning_variables1() {
    let variable_1: i32 = 10; // Nombre de la varaible "variable_1", Tipo de la variable. (DATO EXPLICITO, NOT INFERIDO)
    let variable_2 = 100; // Nombre de la varaible "variable_2", Tipo de la variable. (DATO IMPLICITO, INFERIDO)
    println!("La variable 1 vale: {}", variable_1);
    println!("La variable 2 vale: {}", variable_2);
}

fn main() {
    learning_variables1();
}
