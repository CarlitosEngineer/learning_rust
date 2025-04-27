// - 2.4 - Data Structures - Vector -->
fn testing_datatypes_vector() {
    // Creando vectores
    let vector1: Vec<i32> = vec![1, 2, 3, 4, 5]; // vector inicializado con valores
    let mut vector2: Vec<i32> = Vec::new();       // vector vacío mutable
    vector2.push(10);
    vector2.push(20);
    vector2.push(30);

    let vector3 = vec![0; 5]; // vector con 5 ceros

    println!("testing_datatypes_vector - vector1: {:?}", vector1);
    println!("testing_datatypes_vector - vector2: {:?}", vector2);
    println!("testing_datatypes_vector - vector3: {:?}", vector3);

    // Acceso individual
    println!(
        "testing_datatypes_vector - acceso individual: {}, {}, {}",
        vector1[0], vector1[1], vector1[2]
    );
}