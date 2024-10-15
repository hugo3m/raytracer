use crate::math::vec::Vec3;

pub struct LightAmbient {
    pub intensity: f64,
}

impl LightAmbient {
    pub fn new(intensity: f64) -> Self {
        LightAmbient { intensity }
    }

    pub fn compute(&self) -> f64 {
        return self.intensity;
    }
}

pub struct LightPoint {
    pub intensity: f64,
    pub position: Vec3,
}

impl LightPoint {
    pub fn new(intensity: f64, position: Vec3) -> Self {
        LightPoint {
            intensity,
            position,
        }
    }

    pub fn compute(&self, point: Vec3, normal: Vec3) -> f64 {
        let direction = self.position - point;
        return self.intensity * (direction.dot(normal) / (normal.norm() * direction.norm()));
    }
}

pub struct LightDirectional {
    pub intensity: f64,
    pub direction: Vec3,
}

impl LightDirectional {
    pub fn new(intensity: f64, direction: Vec3) -> Self {
        LightDirectional {
            intensity,
            direction,
        }
    }

    pub fn compute(&self, normal: Vec3) -> f64 {
        return self.intensity
            * (self.direction.dot(normal) / (normal.norm() * self.direction.norm()));
    }
}
