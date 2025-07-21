/*

R/

Usa este comando para ejecutar este código: cargo run --bin 8-DataTypes_String

Fuente consultada:

*/

// - 2.4 - Data Structures - String -->
fn testing_datatypes_string() {
    // Creando Strings
    let string1 = String::from("Hola mundo"); // usando from
    let mut string2 = "Rust".to_string(); // usando to_string y mutable
    string2.push_str(" es genial!"); // agregar más texto
    string2.push('🚀'); // agregar un solo carácter

    let string3 = String::new(); // string vacío

    println!("testing_datatypes_string - string1: {}", string1);
    println!("testing_datatypes_string - string2: {}", string2);
    println!("testing_datatypes_string - string3 (vacía): '{}'", string3);

    // Acceso a propiedades
    println!(
        "testing_datatypes_string - longitudes: string1 = {}, string2 = {}, string3 = {}",
        string1.len(),
        string2.len(),
        string3.len()
    );

    // Reemplazo de contenido
    let replaced = string1.replace("mundo", "Rustacean");
    println!("testing_datatypes_string - reemplazo: {}", replaced);

    // String ➔ cadena dinámica, vive en heap.
    // &str ➔ slice de string, prestado, más liviano.
}

fn main() {
    testing_datatypes_string();
}
