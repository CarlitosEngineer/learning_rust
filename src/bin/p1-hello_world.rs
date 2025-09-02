pub fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    println!("{}", hello());
}

// cargo run --bin p1-hello_world
