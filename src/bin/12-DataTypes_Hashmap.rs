// HashMap -->
use std::collections::HashMap;

fn testing_datatypes_hashmap() {
    // Crear un HashMap vacío
    let mut scores = HashMap::new();

    // Insertar valores
    scores.insert(String::from("Azul"), 10);
    scores.insert(String::from("Rojo"), 50);

    // Crear un HashMap con colecciones (de un vector de tuplas)
    let teams = vec!["Amarillo", "Verde"];
    let initial_scores = vec![30, 40];

    let hashmap_from_vec: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Acceder a un valor
    if let Some(score) = scores.get("Azul") {
        println!("Puntaje del equipo Azul: {}", score);
    }

    // Mostrar todos los valores
    println!("testing_datatypes_hashmap - scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("testing_datatypes_hashmap - hashmap_from_vec:");
    for (key, value) in &hashmap_from_vec {
        println!("{}: {}", key, value);
    }
}
