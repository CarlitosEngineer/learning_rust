// - 2.4 - Data Structures - Tuple -->
fn testing_datatypes_tuple() {
    let tuple1: (i32, f64, u8) = (500, 6.4, 1);
    let tuple2: (char, bool, f32) = ('R', true, 3.14);

    // Desestructuración (extraemos los valores de la tupla)
    let (x, y, z) = tuple1;
    let (a, b, c) = tuple2;

    println!("testing_datatypes_tuple - tuple1: ({}, {}, {})", x, y, z);
    println!("testing_datatypes_tuple - tuple2: ({}, {}, {})", a, b, c);

    // Acceso directo por índice
    println!(
        "testing_datatypes_tuple - acceso por índice: {} {} {}",
        tuple1.0, tuple1.1, tuple1.2
    );
    println!(
        "testing_datatypes_tuple - acceso por índice: {} {} {}",
        tuple2.0, tuple2.1, tuple2.2
    );
}