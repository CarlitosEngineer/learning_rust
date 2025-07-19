# Rust Cheat Sheet - Language

## 📊 Tipos de Datos en Rust

Rust tiene dos grandes categorías de tipos de datos: **Escalares** y **Compuestos**.

- Los tipos escalares representan un único valor.
- Los tipos compuestos agrupan múltiples valores en una sola estructura.

¡Perfecto! Vamos a completar tu **Tabla 1** con TODO lo que falta — incluyendo **Vectores**, **HashMap**, **HashSet**, **LinkedList**, **BinaryHeap**, **Stack**, **Queue**, y luego en sección **Advanced** los tipos como `Rc`, `Arc`, `Mutex`, `RwLock`, `Channels`, `BTreeMap`, `BTreeSet` 🔥.

Aquí tienes la **tabla completada** al 100%:

---

### 2.4 - Data Structures (Tipos de Datos y Estructuras de Datos en Rust) 🦀

| **Categoría**          | **Tipo**                                   | **Ejemplo**                                                               | **Descripción**                                         |
| ---------------------- | ------------------------------------------ | ------------------------------------------------------------------------- | ------------------------------------------------------- |
| **Enteros** (Signed)   | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `let signed_number: i32 = -100;`                                          | Enteros con signo (positivos y negativos).              |
| **Enteros** (Unsigned) | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `let unsigned_number: u32 = 100;`                                         | Enteros sin signo (solo positivos).                     |
| **Flotantes**          | `f32`, `f64`                               | `let float_number: f32 = 3.14;`                                           | Números con decimales (punto flotante).                 |
| **Booleanos**          | `bool`                                     | `let is_rust_fun: bool = true;`                                           | Representa `true` o `false`.                            |
| **Caracteres**         | `char`                                     | `let letter: char = 'R';`                                                 | Almacena un único carácter Unicode.                     |
| **Cadenas**            | `&str`, `String`                           | `let dynamic_text: String = String::from("Hello");`                       | Texto estático (`&str`) o dinámico (`String`).          |
| **Arreglos**           | `[T; N]`                                   | `let numbers: [i32; 3] = [1, 2, 3];`                                      | Conjunto de elementos del mismo tipo y tamaño fijo.     |
| **Tuplas**             | `(T1, T2, ...)`                            | `let tuple: (i32, f64, char) = (42, 3.14, 'x');`                          | Conjunto de valores de diferentes tipos y tamaño fijo.  |
| **Vectores**           | `Vec<T>`                                   | `let numbers = vec![1, 2, 3];`                                            | Colección dinámica de elementos del mismo tipo.         |
| **HashMap**            | `HashMap<K, V>`                            | `let mut map = HashMap::new(); map.insert("key", 42);`                    | Mapa de pares clave-valor desordenado.                  |
| **HashSet**            | `HashSet<T>`                               | `let mut set = HashSet::new(); set.insert(5);`                            | Conjunto de valores únicos sin orden.                   |
| **LinkedList**         | `LinkedList<T>`                            | `let mut list = LinkedList::new(); list.push_back(1);`                    | Lista doblemente enlazada.                              |
| **BinaryHeap**         | `BinaryHeap<T>`                            | `let mut heap = BinaryHeap::new(); heap.push(1);`                         | Montículo binario para obtener el mayor/menor elemento. |
| **Stack** (LIFO)       | `Vec<T>` como pila                         | `let mut stack = Vec::new(); stack.push(1); stack.pop();`                 | Implementación típica de pila (Last In, First Out).     |
| **Queue** (FIFO)       | `VecDeque<T>`                              | `let mut queue = VecDeque::new(); queue.push_back(1); queue.pop_front();` | Cola de doble extremo eficiente.                        |
| **Referencias**        | `&T`                                       | `let reference: &i32 = &signed_number;`                                   | Referencia a otro valor sin tomar posesión.             |
| **Punteros en Heap**   | `Box<T>`                                   | `let boxed_value: Box<i32> = Box::new(128);`                              | Guarda un valor en el heap en lugar de en la pila.      |

#### Extra!

| **Tamaño**  | **Con signo (`i`)**                                            | **Sin signo (`u`)**                    |
| ----------- | -------------------------------------------------------------- | -------------------------------------- |
| **8-bit**   | `i8`  (-128 a 127)                                             | `u8`  (0 a 255)                        |
| **16-bit**  | `i16` (-32,768 a 32,767)                                       | `u16` (0 a 65,535)                     |
| **32-bit**  | `i32` (-2,147,483,648 a 2,147,483,647)                         | `u32` (0 a 4,294,967,295)              |
| **64-bit**  | `i64` (-9,223,372,036,854,775,808 a 9,223,372,036,854,775,807) | `u64` (0 a 18,446,744,073,709,551,615) |
| **128-bit** | `i128` (valores muy grandes)                                   | `u128` (valores muy grandes)           |
| **"arch"**  | `isize` (Depende de la arquitectura)                           | `usize` (Depende de la arquitectura)   |

---

### 2.4.1 - Tipos Avanzados 🚀

| **Categoría**                       | **Tipo**         | **Ejemplo**                                          | **Descripción**                                                          |
| ----------------------------------- | ---------------- | ---------------------------------------------------- | ------------------------------------------------------------------------ |
| **Mapa Ordenado**                   | `BTreeMap<K, V>` | `let mut map = BTreeMap::new(); map.insert(1, "a");` | Mapa de clave-valor ordenado por clave.                                  |
| **Set Ordenado**                    | `BTreeSet<T>`    | `let mut set = BTreeSet::new(); set.insert(3);`      | Conjunto de valores únicos, ordenados.                                   |
| **Contador de Referencias**         | `Rc<T>`          | `let a = Rc::new(5); let b = Rc::clone(&a);`         | Conteo de referencias de punteros para múltiples dueños en un solo hilo. |
| **Contador Atómico de Referencias** | `Arc<T>`         | `let a = Arc::new(5); let b = Arc::clone(&a);`       | Conteo de referencias de punteros para múltiples dueños entre hilos.     |
| **Mutex**                           | `Mutex<T>`       | `let m = Mutex::new(5);`                             | Protección de datos para acceso exclusivo entre hilos.                   |
| **RwLock**                          | `RwLock<T>`      | `let lock = RwLock::new(5);`                         | Permite múltiples lecturas o una única escritura simultánea.             |
| **Canales**                         | `mpsc::channel`  | `let (tx, rx) = mpsc::channel();`                    | Comunicación segura entre hilos (Multiple Producer Single Consumer).     |

# 📌 Apuntes de Rust 🚀

# 9. **Ciclo Loop**   

En **Rust** existen tres tipos principales de **loops**:

1. **`loop`** → Bucle infinito que se rompe con `break`.
2. **`while`** → Se ejecuta mientras una condición sea `true`.
3. **`for`** → Itera sobre una colección.

| **Comando** | **Uso**                                                   |
| ----------- | --------------------------------------------------------- |
| `loop`      | Crea un **bucle infinito** hasta que se use `break`.      |
| `while`     | Ejecuta el loop **mientras la condición sea `true`**.     |
| `for`       | Itera sobre un **rango o colección** (`1..=5`, `vec![]`). |
| `break`     | **Rompe** el loop antes de que termine.                   |
| `continue`  | **Salta una iteración** y sigue con la siguiente.         |