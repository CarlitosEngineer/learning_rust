// - 2.4 - Data Structures - Array -->
fn testing_datatypes_array() {
    // Declarando arrays de enteros
    let array1: [i32; 5] = [1, 2, 3, 4, 5]; // array de 5 elementos
    let array2: [i32; 5] = [0; 5]; // array con 5 ceros
    let array3: [i32; 3] = [10, 20, 30]; // array más pequeño

    println!("testing_datatypes_array - array1: {:?}", array1);
    println!("testing_datatypes_array - array2: {:?}", array2);
    println!("testing_datatypes_array - array3: {:?}", array3);

    // Acceso individual
    println!(
        "testing_datatypes_array - acceso individual: {}, {}, {}",
        array1[0], array1[1], array1[2]
    );
}

fn main() {
    testing_datatypes_array();
}