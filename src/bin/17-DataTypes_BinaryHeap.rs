// BinaryHeap -->
use std::collections::BinaryHeap;

fn testing_datatypes_binaryheap() {
    // Crear un BinaryHeap vacío
    let mut heap = BinaryHeap::new();

    // Insertar valores
    heap.push(4);
    heap.push(8);
    heap.push(1);
    heap.push(6);

    println!("testing_datatypes_binaryheap - heap completo:");
    for value in heap.iter() {
        println!("{}", value);
    }

    // Ver el valor máximo (peek no elimina)
    if let Some(max) = heap.peek() {
        println!("Valor máximo en el heap: {}", max);
    }

    // Eliminar el valor máximo (pop elimina el mayor elemento)
    if let Some(popped) = heap.pop() {
        println!("Elemento removido (máximo): {}", popped);
    }

    println!("testing_datatypes_binaryheap - después de pop:");
    for value in heap.iter() {
        println!("{}", value);
    }
}

// use std::cmp::Reverse;

// let mut min_heap = BinaryHeap::new();
// min_heap.push(Reverse(10));
// min_heap.push(Reverse(5));
// min_heap.push(Reverse(20));

// // Para obtener el menor elemento
// if let Some(Reverse(min)) = min_heap.peek() {
//     println!("Valor mínimo en el min-heap: {}", min);
// }
