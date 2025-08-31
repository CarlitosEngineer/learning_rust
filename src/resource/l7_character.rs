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

fn main() {
    testing_datatypes_character();
}

/*

Source: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type

📌 Run this code using: cargo run --bin 7-DataTypes_Character

*/
