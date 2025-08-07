fn learning_scope1() {
    let variable_1 = 10;
    const VARIABLE_CONST2: u32 = 101;
    {
        let variable_2 = 100;
        println!(
            "variable_1 dentro del sub bloque : {}, variable_2: {}", // LA VARIABLE 1 SE PUEDE IMPRIMIR
            variable_1, variable_2
        );
    }
    println!("variable_1: {} despues del bloque 2", variable_1); // LA VARIABLE 1 SE PUEDE IMPRIMIR
    println!("El valor 3 es: {}", VARIABLE_CONST2);
    // println!("{}", variable_2); // LA VARIABLE 2 NO SE PUEDE IMPRIMIR
}

fn learning_shadowing1() {
    let variable_3 = 5; // AQUI LA VARIABLE SE CREA POR PRIMERA VEZ
    let variable_3 = variable_3 + 1; // AQUI LA VARIABLE SE CREA POR SEGUNDA VEZ
    let variable_3 = variable_3 * 2; // AQUI LA VARIABLE SE CREA POR TERCERA VEZ
    println!("variable_3 = {}", variable_3); // 12
    // EL SHADOWING NO ES MUTABILIDAD; ES LA REDECLARACIÓN DE UNA VARIABLE
}

fn main() {
    learning_scope1();
    learning_shadowing1();
}

/*

¿Que es el scope?

* Scope (ámbito): Es el alcance que tiene una variable en un entorno o ambiente.
* Entorno o Bloque: Es un bloque en el codigo.
* El Shadowing (sombreado): es la redeclracion de una variable con el mismo nombre, creando una nueva.

Usa este comando para ejecutar este código: cargo run --bin 3-Scope_Shadowing

Fuente consultada:

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

*/