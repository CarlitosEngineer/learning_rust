// HashSet -->
use std::collections::HashSet;

fn testing_datatypes_hashset() {
    // Crear un HashSet vacío
    let mut set = HashSet::new();

    // Insertar valores
    set.insert("Azul");
    set.insert("Rojo");
    set.insert("Verde");
    set.insert("Azul"); // No se va a duplicar

    println!("testing_datatypes_hashset - set completo:");
    for color in &set {
        println!("{}", color);
    }

    // Verificar si un valor está presente
    if set.contains("Rojo") {
        println!("Rojo está en el set");
    } else {
        println!("Rojo NO está en el set");
    }

    // Remover un elemento
    set.remove("Verde");

    println!("testing_datatypes_hashset - después de remover Verde:");
    for color in &set {
        println!("{}", color);
    }
}