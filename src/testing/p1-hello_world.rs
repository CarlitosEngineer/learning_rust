// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    hello();
}

// cargo run src/testing/p1-hello_world.rs
