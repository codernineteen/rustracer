pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contain(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }
    pub fn surround(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }

    pub fn clamp(&self, t: f64) -> f64 {
        if t < self.min {
            self.min
        } else if t > self.max {
            self.max
        } else {
            t
        }
    }

    pub fn update(&mut self, min: f64, max: f64) {
        self.min = min;
        self.max = max;
    }
}
