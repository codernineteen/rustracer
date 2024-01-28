use rand::{thread_rng, Rng};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Color3 = Vector3;

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

    pub fn invert(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
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

    pub fn normalize(&self) -> Self {
        Vector3::new(self.x, self.y, self.z) / self.length()
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/*
   impl block for Arithmetic operators
*/
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

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
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
        assert_ne!(rhs, 0.0);
        self.mul(1.0 / rhs)
    }
}

/*
    Test suite for Vector3
*/

#[cfg(test)]
mod tests {
    use super::*;
    // arithmetic tests
    #[test]
    fn test_add() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1 - v2;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let result = v1 * v2;
        let result2 = v2 * 2.0;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, 18.0);
        assert_eq!(result2.x, 8.0);
        assert_eq!(result2.y, 10.0);
        assert_eq!(result2.z, 12.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };
        let result = v1 / v2;
        let result2 = v2 / 2.0;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.25);
        assert_eq!(result.z, 0.25);
        assert_eq!(result2.x, 2.0);
        assert_eq!(result2.y, 2.0);
        assert_eq!(result2.z, 2.0);
    }

    // vector operation tests
    #[test]
    fn test_dot() {
        let v1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let v2 = Vector3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };
        let result = Vector3::dot(&v1, &v2);
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_unit() {
        let v = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 4.0,
        };
        let result = v.normalize();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_length() {
        let v = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 4.0,
        };

        assert_eq!(v.length(), 4.0);
        assert_eq!(v.length_squared(), 16.0);
    }

    #[test]
    fn test_default() {
        let v = Vector3::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }
}
