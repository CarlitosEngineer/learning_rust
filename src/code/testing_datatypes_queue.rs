// Queue -->
use std::collections::VecDeque;

fn testing_datatypes_queue() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Agregar elementos al final
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    println!("testing_datatypes_queue - estado inicial:");
    for value in &queue {
        println!("{}", value);
    }

    // Ver el primer elemento (sin eliminar)
    if let Some(front) = queue.front() {
        println!("Primer elemento: {}", front);
    }

    // Sacar elementos del frente
    if let Some(removed) = queue.pop_front() {
        println!("Elemento removido: {}", removed);
    }

    println!("testing_datatypes_queue - después de pop_front:");
    for value in &queue {
        println!("{}", value);
    }
}
