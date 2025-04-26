

fn main() {
    // - Enteros (con y sin signo)
    let signed_number: i32 = -100; 
    let unsigned_number: u32 = 100;
    let arch_number: usize = 1024; 

    // - Flotantes
    let float_number: f32 = 3.14; 
    let double_number: f64 = 2.718;

    // - Booleanos
    let is_rust_fun: bool = true; 

    // - Caracteres
    let letter: char = 'R'; 
    let emoji: char = '🚀';

    // - Cadenas de texto
    let static_text: &str = "Hello, Rust!"; // Cadena inmutable
    let dynamic_text: String = String::from("This is a String!"); // Cadena mutable

    // - Arreglos y Tuplas
    let numbers: [i32; 3] = [1, 2, 3]; // Array de 3 elementos
    let tuple: (i32, f64, char) = (42, 3.14, 'x'); // Tupla de diferentes tipos

    // - Referencias y punteros
    let reference: &i32 = &signed_number; // Referencia a un entero
    let boxed_value: Box<i32> = Box::new(128); // Puntero en el heap

    // - Imprimir valores
    println!("Signed: {}", signed_number);
    println!("Unsigned: {}", unsigned_number);
    println!("Arch-based: {}", arch_number);
    println!("Float: {}", float_number);
    println!("Double: {}", double_number);
    println!("Boolean: {}", is_rust_fun);
    println!("Char: {}, Emoji: {}", letter, emoji);
    println!("Static text: {}", static_text);
    println!("Dynamic text: {}", dynamic_text);
    println!("Array: {:?}", numbers);
    println!("Tuple: {:?}", tuple);
    println!("Reference: {}", reference);
    println!("Boxed Value: {}", boxed_value);
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 07 -- >>
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

use std::io; // Importar io para entrada de datos

fn main() {
    println!("Por favor introduce tu nombre: ");

    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre).unwrap();    
    nombre = nombre.trim().to_string();

    // Obtener la edad de la consola    
    println!("Por favor introduce tu edad: ");    

    let mut edad = String::new();
    io::stdin().read_line(&mut edad).unwrap();

    // Convertir la edad a un número    
    let edad_int: u8 = edad.trim().parse().unwrap();

    println!("Hola, bienvenido o bienvenida {} de {} años", nombre, edad_int);
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 07; MI APORTE COMPLEMENTARIO AL CODIGO
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

fn main() {
    // - 1. Definiendo Strings
    let static_str: &str = "Hola, Rust!";  // Cadena inmutable (&str)
    let mut dynamic_string = String::from("¡Bienvenido!");  // String dinámico

    // - 2. Concatenación
    let name = "Carlos";
    let greeting = format!("{}, {}!", static_str, name); // Usando format!
    
    // - 3. Agregar texto a un String
    dynamic_string.push_str(" Vamos a programar en Rust."); // Agregar una cadena
    dynamic_string.push('✨'); // Agregar un solo carácter

    // - 4. Convertir entre tipos de String
    let from_static = static_str.to_string(); // De &str a String
    let from_dynamic: &str = &dynamic_string; // De String a &str

    // - 5. Reemplazo y manipulación
    let replaced = dynamic_string.replace("Rust", "el mejor lenguaje");
    let uppercased = dynamic_string.to_uppercase();
    
    // - 6. División y recorte
    let trimmed = "   Rust es genial!   ".trim(); // Elimina espacios en los extremos
    let split_example: Vec<&str> = "Rust,Python,JavaScript".split(',').collect();

    // - 7. Iterar sobre los caracteres
    println!("Iterando sobre caracteres:");
    for ch in dynamic_string.chars() {
        print!("{} ", ch);
    }
    println!(); // Salto de línea

    // - 8. Obtener longitud
    let length = dynamic_string.len(); // En bytes
    let char_count = dynamic_string.chars().count(); // En caracteres

    // - 9. Acceder a partes de un String (Slices)
    let slice = &dynamic_string[0..10]; // Extrae los primeros 10 bytes

    // - 10. Imprimir resultados
    println!("\n--- Resultados ---");
    println!("01 String estático: {}", static_str);
    println!("02 String dinámico: {}", dynamic_string);
    println!("03 Concatenación: {}", greeting);
    println!("04 Convertido de &str a String: {}", from_static);
    println!("05 Convertido de String a &str: {}", from_dynamic);
    println!("06 Reemplazo: {}", replaced);
    println!("07 Mayúsculas: {}", uppercased);
    println!("08 Recorte: '{}'", trimmed);
    println!("09 División: {:?}", split_example);
    println!("10 Longitud en bytes: {}", length);
    println!("11 Número de caracteres: {}", char_count);
    println!("12 Slice del String: {}", slice);
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 08 -- >>
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

fn main() {
    println!("Por favor Introduce tu edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // cambiar edad
    let edad_int : u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int != 30 {
        println!("Puedes entrar a la discoteca");
    } else if edad_int == 30 {
        println!("No admitimos persoans exaamente de 30 años");
    }
    else{
        println!("Eres menor de edad todavia");
    }
    println!("Tienes {} años", edad_int);
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 08; MI APORTE COMPLEMENTARIO AL CODIGO
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

// - 1. Uso básico de `if` y `else`
fn ejemplo_if_else() {
    let edad = 20;

    if edad >= 18 {
        println!("Eres mayor de edad.");
    } else {
        println!("Eres menor de edad.");
    }
}

// - 2. Uso de `else if` para múltiples condiciones
fn ejemplo_else_if() {
    let temperatura = 30;

    if temperatura > 35 {
        println!("Hace mucho calor!");
    } else if temperatura > 25 {
        println!("El clima es agradable.");
    } else {
        println!("Hace frío.");
    }
}

// - 3. Condicional `if` en una expresión (Asignación con `if`)
fn ejemplo_if_expresion() {
    let edad = 16;
    let mensaje = if edad >= 18 { "Adulto" } else { "Menor de edad" };
    println!("Estado: {}", mensaje);
}

// - 4. Uso de `match` como alternativa a `if-else`
fn ejemplo_match() {
    let edad = 18;

    match edad {
        0..=17 => println!("Eres menor de edad."),
        18..=29 => println!("Eres un adulto joven."),
        30 => println!("Tienes 30 años exactamente."),
        _ => println!("Edad no categorizada."),
    }
}

// - 5. Uso de `if let` (para patrones más simples)
fn ejemplo_if_let() {
    let opcion: Option<i32> = Some(42);

    if let Some(valor) = opcion {
        println!("El valor dentro de Some es: {}", valor);
    } else {
        println!("No hay ningún valor.");
    }
}

// - 6. Uso de `while let` con patrones
fn ejemplo_while_let() {
    let mut numeros = vec![1, 2, 3];

    while let Some(num) = numeros.pop() {
        println!("Número extraído: {}", num);
    }
}

// - Llamar a las funciones en `main()`
fn main() {
    ejemplo_if_else();
    ejemplo_else_if();
    ejemplo_if_expresion();
    ejemplo_match();
    ejemplo_if_let();
    ejemplo_while_let();
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 09 -- >>
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

fn main() {
    // Dos numeros que vamos a sumar
    let numero_1 = 120;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;

    loop {
        // Mostrar los dos numeros en pantalla
        println!("Por favor escribir la suma de {} y {}: ", numero_1, numero_2);

        // Obtener del usuario el numero que representa la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int : i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} no es correcto por favor intentalo de nuevo", suma_usuario_int);
        }
    }

}

fn main() {
    loop {
        println!("Ingrese 123 para detener el loop:");
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number).unwrap();
        let number_int: i32 = number.trim().parse().unwrap();

        if number_int == 123 {
            break;
        }
    }
}

/*
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
        CODIGO - CLASE 09; MI APORTE COMPLEMENTARIO AL CODIGO
    ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- // ----- //
*/

/*

    En **Rust** existen tres tipos principales de **loops**:

        1. **`loop`** → Bucle infinito que se rompe con `break`.
        2. **`while`** → Se ejecuta mientras una condición sea `true`.
        3. **`for`** → Itera sobre una colección.

    | **Comando**  | **Uso** |
    |-------------|---------------------------|
    | `loop`      | Crea un **bucle infinito** hasta que se use `break`. |
    | `while`     | Ejecuta el loop **mientras la condición sea `true`**. |
    | `for`       | Itera sobre un **rango o colección** (`1..=5`, `vec![]`). |
    | `break`     | **Rompe** el loop antes de que termine. |
    | `continue`  | **Salta una iteración** y sigue con la siguiente. |

*/

fn main() {
    // - 1. Bucle infinito con `loop`
    let mut contador = 0;
    loop {
        println!("Contador: {}", contador);
        contador += 1;
        if contador == 3 {
            break; // Rompe el loop cuando contador llega a 3
        }
    }

    // - 2. Bucle `while`
    let mut numero = 5;
    while numero > 0 {
        println!("Número: {}", numero);
        numero -= 1;
    }

    // - 3. Bucle `for` con rango
    for i in 1..=5 {  // Rango de 1 a 5 (incluye el 5)
        println!("Iteración: {}", i);
    }

    // - 4. `for` sobre una colección
    let numeros = vec![10, 20, 30];
    for num in &numeros {
        println!("Número en la colección: {}", num);
    }

    // - 5. `for` con `enumerate` (para obtener índice y valor)
    for (indice, valor) in numeros.iter().enumerate() {
        println!("Índice: {}, Valor: {}", indice, valor);
    }

    // - 6. Uso de `continue` (para saltar una iteración)
    for n in 1..=5 {
        if n == 3 {
            continue; // Salta el número 3 y sigue con la siguiente iteración
        }
        println!("Número sin 3: {}", n);
    }
}
