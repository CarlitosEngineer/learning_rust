fn testing_datatypes_integers() {
    // Unsigne
    let variable_u1: u8 = 255; // de 0 a 255
    let variable_u2: u16 = 65535; // de 0 a 65,535
    let variable_u3: u32 = 4294967295; // de 0 a 4,294,967,295
    let variable_u4: u64 = 18446744073709551615; // de 0 a 18,446,744,073,709,551,615
    let variable_u5: u128 = 340282366920938463463374607431768211455; // de 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455
    let variable_u6: usize = 18446744073709551615; // en un sistema de 64 bits
    // - 32 bits: 0 a 4,294,967,295 (usize)
    // - 64 bits: 0 a 18,446,744,073,709,551,615 (usize)

    // Signe
    let variable_i1: i8 = -128; // -128 a 127
    let variable_i2: i16 = -32768; // -32,768 a 32,767
    let variable_i3: i32 = -2147483648; // -2,147,483,648 a 2,147,483,647
    let variable_i4: i64 = -9223372036854775808; // -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
    let variable_i5: i128 = -170141183460469231731687303715884105728; // -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727
    let variable_i6: isize = -9223372036854775808; // en un sistema de 64 bits
    // - 32 bits: -2,147,483,648 a 2,147,483,647  (isize)
    // - 64 bits: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 (isize)

    // Tambien se pueden nombrar las variables de otras forma, esta se llama sufijo.
    let variable_8 = 100u8; // la variable se llama "variable_8", valor: 100, de tipo u8
    let variable_9 = -20i16;

    // i32 es el tipo entero por defecto si no se especifica
    let variable_7 = 42; // i32 por defecto (IMPLICITO)

    // Imprimiendo, valores sin signo
    println!(
        "testing_datatypes_integers U: {}, {}, {}, {}, {}, {}",
        variable_u1, variable_u2, variable_u3, variable_u4, variable_u5, variable_u6
    );

    // Imprimiendo, valores con signo
    println!(
        "testing_datatypes_integers I: {}, {}, {}, {}, {}, {}",
        variable_i1, variable_i2, variable_i3, variable_i4, variable_i5, variable_i6
    );

    // Usando sufijos
    println!(
        "testing_datatypes_integers U: {}, {} ",
        variable_8, variable_9
    );

    // Usando valor por defecto
    println!("testing_datatypes_integers U: {} ", variable_7);
}

fn main() {
    testing_datatypes_integers();
}

/*

- Como ya sabemos, Las variables tiene un "TIPO DE DATO".
- estas variables al tener un "TIPO DE DATO", solo pueden almacenar un rango de valores.
- isize/usize: varian de rango segun el sistema en el que se programe
- Cuando no se especifica el tipo de entero, rust por defecto usa i32
- Existe otra forma de declrar una varaible usando "sufijos"

Usa este comando para ejecutar este código: cargo run --bin 4-DataTypes_Integers

- The Rust Programming Language, Capítulo 3.1: Variables and Mutability
    https://doc.rust-lang.org/book/ch03-02-data-types.html

*/
