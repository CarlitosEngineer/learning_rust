// - 2.4 - Data Structures - Integers -->
fn testing_datatypes_integers() {
    let value_i1: i8 = -128; // -128 a 127
    let value_i2: i16 = -32768; // -32,768 a 32,767
    let value_i3: i32 = -2147483648; // -2,147,483,648 a 2,147,483,647
    let value_i4: i64 = -9223372036854775808; // -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
    let value_i5: i128 = -170141183460469231731687303715884105728; // -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727
    let value_i6: isize = -9223372036854775808; // en un sistema de 64 bits    

    let value_u1: u8 = 255; // de 0 a 255
    let value_u2: u16 = 65535; // de 0 a 65,535
    let value_u3: u32 = 4294967295; // de 0 a 4,294,967,295
    let value_u4: u64 = 18446744073709551615; // de 0 a 18,446,744,073,709,551,615
    let value_u5: u128 = 340282366920938463463374607431768211455; // de 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455
    let value_u6: usize = 18446744073709551615; // en un sistema de 64 bits

    println!(
        "testing_datatypes_integers I: {}, {}, {}, {}, {}, {}",
        value_i1, value_i2, value_i3, value_i4, value_i5, value_i6
    );
    println!(
        "testing_datatypes_integers U: {}, {}, {}, {}, {}, {}",
        value_u1, value_u2, value_u3, value_u4, value_u5, value_u6
    );
}