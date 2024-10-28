use std::ops;

/// Find the reflection vector from
/// * the direction that hit the surface
/// * the normal of the surface
pub fn reflection(direction: &Vec3, normal: &Vec3) -> Vec3 {
    return *direction - *normal * 2.0 * normal.dot(*direction);
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
    /// Compute dot from one vector to another
    pub fn dot(self, _rhs: Self) -> f64 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }
    /// Get the norm
    pub fn norm(self) -> f64 {
        return self.dot(self).sqrt();
    }
    /// Get the normalize vector
    pub fn normalize(self) -> Vec3 {
        let norm = self.norm();
        if norm > 0.0 {
            return Vec3::new(self.x, self.y, self.z) / self.norm();
        }
        return Vec3::new(self.x, self.y, self.z);
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    /// Add one vector to another
    fn add(self, _rhs: Self) -> Self {
        return Self::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z);
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    /// Negate the vector
    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y, -self.z);
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    /// Subtract one vector to another
    fn sub(self, _rhs: Self) -> Self {
        return Self::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z);
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    /// Multiply one vector by f64
    fn mul(self, _rhs: f64) -> Self {
        return Self::new(self.x * _rhs, self.y * _rhs, self.z * _rhs);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    /// Divide one vector by f64
    fn div(self, _rhs: f64) -> Self {
        return Self::new(self.x / _rhs, self.y / _rhs, self.z / _rhs);
    }
}
