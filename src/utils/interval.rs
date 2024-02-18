#[derive(Copy, Clone)]
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

    pub fn wrap_intervals(a: &Interval, b: &Interval) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta * 0.5;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.max(self.min).min(self.max)
    }
}
