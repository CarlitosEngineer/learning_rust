// - 2.4 - Data Structures - Boolean -->
fn testing_datatypes_boolean() {
    let value_b1: bool = true; // verdadero
    let value_b2: bool = false; // falso

    println!("testing_datatypes_boolean: {}, {}", value_b1, value_b2);
}

// Character -->
fn testing_datatypes_character() {
    let value_c1: char = 'A'; // letra mayúscula
    let value_c2: char = 'z'; // letra minúscula
    let value_c3: char = '0'; // número como carácter
    let value_c4: char = '$'; // símbolo especial
    let value_c5: char = '❤'; // emoji corazón (unicode)
    let value_c6: char = '🚀'; // emoji cohete (unicode)

    println!(
        "testing_datatypes_character: {}, {}, {}, {}, {}, {}",
        value_c1, value_c2, value_c3, value_c4, value_c5, value_c6
    );
}