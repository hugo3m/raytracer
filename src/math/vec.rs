use std::ops;

struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
/// Vec3 of f64
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, _rhs: Self) -> f64 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }

    pub fn norm(self) -> f64 {
        return self.dot(self).sqrt();
    }

    pub fn normalize(self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z) / self.norm();
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        return Self::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z);
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y, -self.z);
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        return Self::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z);
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        return Self::new(self.x * _rhs, self.y * _rhs, self.z * _rhs);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        return Self::new(self.x / _rhs, self.y / _rhs, self.z / _rhs);
    }
}
