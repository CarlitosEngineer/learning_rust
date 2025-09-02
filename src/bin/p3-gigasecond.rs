// use time::macros::datetime;
// use time::{Duration, PrimitiveDateTime};

use time::macros::datetime;
use time::{Duration, PrimitiveDateTime};

const GIGASECOND: i64 = 1_000_000_000; // creo una constante tantos segundos.

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(GIGASECOND) // sumo la variable con un objeto que vale la constante
}

fn main() {
    let start: PrimitiveDateTime = datetime!(2000-01-01 00:00:00);
    let result: PrimitiveDateTime = after(start);
    println!("{}", result); // para comprobar
}

// cargo run --bin p2-reverse_string
