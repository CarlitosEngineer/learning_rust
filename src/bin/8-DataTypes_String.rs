fn testing_datatypes_string() {
    // Creating Strings
    let string1 = String::from("Hola mundo"); // using String::from
    let mut string2 = "Rust".to_string(); // using to_string and making it mutable
    string2.push_str(" es genial!"); // appending more text
    string2.push('🚀'); // appending a single character

    let string3 = String::new(); // empty string

    println!("testing_datatypes_string - string1: {}", string1);
    println!("testing_datatypes_string - string2: {}", string2);
    println!("testing_datatypes_string - string3 (empty): '{}'", string3);

    // Accessing properties
    println!(
        "testing_datatypes_string - lengths: string1 = {}, string2 = {}, string3 = {}",
        string1.len(),
        string2.len(),
        string3.len()
    );

    // Content replacement
    let replaced = string1.replace("mundo", "Rustacean");
    println!("testing_datatypes_string - replacement: {}", replaced);

    // Notes:
    // String  ➔ dynamic string, heap-allocated.
    // &str    ➔ string slice, borrowed, more lightweight.
}

fn main() {
    testing_datatypes_string();
}

/*
Source: https://doc.rust-lang.org/book/ch08-02-strings.html

📌 Run this code using: cargo run --bin 8-DataTypes_String
*/
