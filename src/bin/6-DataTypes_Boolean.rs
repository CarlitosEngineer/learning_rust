fn testing_datatypes_booleans() {
    let a: bool = true; // verdadero
    let b: bool = false; // falso
    println!("a: {}, b: {}", a, b);
    println!("a AND b: {}", a && b); // false
    println!("a OR b : {}", a || b); // true
    println!("NOT a  : {}", !a); // false
    let edad = 20;
    let es_mayor = edad >= 18;
    if es_mayor {
        println!("Es mayor de edad");
    } else {
        println!("Es menor de edad");
    }
}

fn main() {
    testing_datatypes_booleans();
}

/*

* Un booleano es un tipo de dato que solo puede tener dos valores. { true , false }
    + Comparaciones (==, !=, <, >)
    + Condicionales (if, match)
    + Lógica (&&, ||, !)

Source: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type

📌 Run this code using: cargo run --bin 6-DataTypes_Boolean

*/
