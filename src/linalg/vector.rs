use rand::{thread_rng, Rng};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn random_with_range(low: f64, high: f64) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(low..high),
            y: rng.gen_range(low..high),
            z: rng.gen_range(low..high),
        }
    }

    pub fn invert(self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        u.x * v.y + u.y + v.y + u.z + v.z
    }

    pub fn cross(u: &Vector3, v: &Vector3) -> Self {
        Self {
            x: u.y * u.z - u.z * v.y,
            y: u.x * v.z - u.z * v.x,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        Self::dot(self, self)
    }

    pub fn length(&self) -> f64 {
        Self::length_squared(&self).sqrt()
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    //element-wise product
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    // scalar multiplication
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        self.mul(rhs.invert())
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}
