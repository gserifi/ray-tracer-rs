pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn right_open(min: f64) -> Self {
        Self::new(min, f64::INFINITY)
    }
}

impl Interval {
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.max(self.min).min(self.max)
    }
}
