use crate::math::vec::Vec3;

pub struct LightComputeInfo {
    pub point: Vec3,
    pub normal: Vec3,
}

impl LightComputeInfo {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        LightComputeInfo { point, normal }
    }
}

pub trait Light {
    fn compute(&self, info: &LightComputeInfo) -> f64;
}

pub struct LightAmbient {
    pub intensity: f64,
}

impl LightAmbient {
    pub fn new(intensity: f64) -> Self {
        LightAmbient { intensity }
    }
}

impl Light for LightAmbient {
    fn compute(&self, _info: &LightComputeInfo) -> f64 {
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
}

impl Light for LightPoint {
    fn compute(&self, info: &LightComputeInfo) -> f64 {
        let direction = self.position - info.point;
        return self.intensity
            * (direction.dot(info.normal) / (info.normal.norm() * direction.norm()));
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
}

impl Light for LightDirectional {
    fn compute(&self, info: &LightComputeInfo) -> f64 {
        return self.intensity
            * (self.direction.dot(info.normal) / (info.normal.norm() * self.direction.norm()));
    }
}
