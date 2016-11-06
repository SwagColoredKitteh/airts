use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn len(self) -> f64 {
        (self * self).sqrt()
    }

    pub fn manhattan_len(self) -> f64 {
        self * self
    }

    pub fn manhattan_distance(self, other: Vec2) -> f64 {
        (other - self).manhattan_len()
    }
    
    pub fn norm(self) -> Vec2 {
        let len = self.len();
        self * (1.0 / len)
    }
    
    pub fn from_angle(angle: f64, len: f64) -> Vec2 {
        Vec2(angle.cos() * len, angle.sin() * len)
    }
    
    pub fn angle(self) -> f64 {
        self.1.atan2(self.0)
    }
    
    pub fn distance_to(self, other: Vec2) -> f64 {
        (other - self).len()
    }
    
    pub fn round(self) -> Vec2 {
        Vec2(self.0.round(), self.1.round())
    }

    pub fn floor(self) -> Vec2 {
        Vec2(self.0.floor(), self.1.floor())
    }
    
    pub fn perp(self) -> Vec2 {
        Vec2(self.1, -self.0)
    }

    pub fn cross(self, other: Vec2) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }

    pub fn neg(self) -> Vec2 {
        Vec2(-self.0, -self.1)
    }

    pub fn vector_to(self, other: Vec2) -> Vec2 {
        other - self
    }
    
    pub fn rotate(self, angle: f64) -> Vec2 {
        let x = self.0;
        let y = self.1;
        let c = angle.cos();
        let s = angle.sin();
        Vec2(x * c - y * s, x * s + y * c)
    }
    
    pub fn move_to(self, target: Vec2, range: f64) -> Vec2 {
        let diff = self.vector_to(target);
        if diff.len() < range {
            target
        }
        else {
            let norm = diff.norm();
            self + norm * range
        }
    }
}

impl Mul for Vec2 {
    type Output = f64;
    fn mul(self, other: Vec2) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f64) -> Vec2 {
        Vec2(self.0 * other, self.1 * other)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = *self - other;
    }
}
