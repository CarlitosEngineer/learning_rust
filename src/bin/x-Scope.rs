fn testing_lesson4() {
    // lesson 4 - Scope (Ámbito)
    let value4 = 5;
    {
        let value5 = 10;
        println!(
            "value4 dentro del sub bloque : {}, value5: {}",
            value4, value5
        );
    }
    // println!("{}", value5); // NO SE PUEDE IMPRIMIR ESTE VALOR
    println!("value4: {} despues del bloque 2", value4);

    let value6 = 5;
    let value6 = value6 + 1; // nueva variable value6
    let value6 = value6 * 2; // otra vez
    println!("value6 = {}", value6); // 12
}

fn main() {
    testing_lesson4();
}
