// - 2.4 - Data Structures - Floats -->
fn testing_datatypes_floats() {
    // Valores normales
    let value_f1: f32 = 3.1415927; // ejemplo f32 positivo
    let value_f2: f32 = -2.7182818; // ejemplo f32 negativo
    let value_f3: f64 = 3.141592653589793; // ejemplo f64 positivo
    let value_f4: f64 = -2.718281828459045; // ejemplo f64 negativo

    // Valores especiales f32
    let inf_f32: f32 = f32::INFINITY; // infinito positivo
    let neg_inf_f32: f32 = f32::NEG_INFINITY; // infinito negativo
    let nan_f32: f32 = f32::NAN; // Not a Number
    let min_f32: f32 = f32::MIN; // valor mínimo representable (negativo más grande)
    let max_f32: f32 = f32::MAX; // valor máximo representable (positivo más grande)

    // Valores especiales f64
    let inf_f64: f64 = f64::INFINITY;
    let neg_inf_f64: f64 = f64::NEG_INFINITY;
    let nan_f64: f64 = f64::NAN;
    let min_f64: f64 = f64::MIN;
    let max_f64: f64 = f64::MAX;

    println!(
        "testing_datatypes_floats - f32 normales: {}, {}",
        value_f1, value_f2
    );
    println!(
        "testing_datatypes_floats - f64 normales: {}, {}",
        value_f3, value_f4
    );
    println!(
        "testing_datatypes_floats - f32 especiales: INF = {}, NEG_INF = {}, NAN = {}, MIN = {}, MAX = {}",
        inf_f32, neg_inf_f32, nan_f32, min_f32, max_f32
    );
    println!(
        "testing_datatypes_floats - f64 especiales: INF = {}, NEG_INF = {}, NAN = {}, MIN = {}, MAX = {}",
        inf_f64, neg_inf_f64, nan_f64, min_f64, max_f64
    );
}

fn main() {
    testing_datatypes_floats();
}