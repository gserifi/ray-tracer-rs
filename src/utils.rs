pub fn random() -> f64 {
    rand::random::<f64>()
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
