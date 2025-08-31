// - 2.4 - Data Structures - Array -->
fn testing_datatypes_array() {
    // Declarando arrays de enteros
    let array1: [i32; 5] = [1, 2, 3, 4, 5]; // Sintaxis para Tipo y Longitud
    let array2: [i32; 5] = [0; 5]; // Inicializar con el Mismo Valor {// [3, 3, 3, 3, 3]}
    println!("testing_datatypes_array - array1: {:?}", array1);
    println!("testing_datatypes_array - array2: {:?}", array2);

    let primero = array1[0]; // acceder a un arreglo
    let segundo = array1[1];
    println!("testing_datatypes_array - array1: {:?}", primero);
    println!("testing_datatypes_array - array1: {:?}", segundo);

    // Acceso individual
    println!(
        "testing_datatypes_array - acceso individual: {}, {}, {}",
        array1[0], array1[1], array1[2]
    );
}

fn main() {
    testing_datatypes_array();
}

/*

* Colección de tamaño fijo
* Elementos del mismo tipo
* Se almacenan en la pila (stack)
* Usa array si el tamaño es fijo.
* Usa vector (Vec<T>) si el tamaño debe ser dinámico.

Source: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type

📌 Run this code using: cargo run --bin 10-DataTypes_Array
*/
