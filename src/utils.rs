pub fn float_eq(a: f64, b: f64) -> bool {
    (b - a).abs() < 1e-6
}
