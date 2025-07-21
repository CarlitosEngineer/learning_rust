// LinkedList -->
use std::collections::LinkedList;

fn testing_datatypes_linkedlist() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // Agregar elementos al final
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Agregar elementos al principio
    list.push_front(0);

    println!("testing_datatypes_linkedlist - lista completa:");
    for value in &list {
        println!("{}", value);
    }

    // Ver el primer y último elemento
    if let Some(first) = list.front() {
        println!("Primer elemento: {}", first);
    }

    if let Some(last) = list.back() {
        println!("Último elemento: {}", last);
    }

    // Eliminar del frente y fondo
    list.pop_front();
    list.pop_back();

    println!("testing_datatypes_linkedlist - después de pop_front y pop_back:");
    for value in &list {
        println!("{}", value);
    }
}
