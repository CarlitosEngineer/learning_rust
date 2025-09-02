pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("{}", reverse("epale"));
}

// cargo run --bin p2-reverse_string
