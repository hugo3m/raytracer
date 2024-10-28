use crate::{material::Material, math::vec::reflection, math::vec::Vec3};

use super::sphere::{find_intersection, Sphere};

fn computer_specular(
    intensity: f64,
    direction: &Vec3,
    info: &LightComputeInfo,
    material: &Material,
) -> f64 {
    // specular use as exponent
    if material.specular < 0.0 {
        return 0.0;
    }
    // maximum reflection from the light into the surface
    let reflected = reflection(&direction, &info.normal);
    // calculate the coefficient of intensity sent back according to the viewer position
    let coeff = reflected.dot(info.direction) / (reflected.norm() * info.direction.norm());
    // if coeff is negative no light is sent back to the viewer from is point of view
    if coeff < 0.0 {
        return 0.0;
    }
    // adjust the cosinus by exponenting it
    return intensity * coeff.powf(material.specular);
}

pub struct LightComputeInfo {
    pub direction: Vec3,
    pub normal: Vec3,
    pub position: Vec3,
    pub is_diffuse: bool,
    pub is_specular: bool,
    pub is_shadow: bool,
}

pub trait Light {
    fn compute(&self, info: &LightComputeInfo, material: &Material, spheres: &Vec<Sphere>) -> f64;
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
    fn compute(
        &self,
        _info: &LightComputeInfo,
        _material: &Material,
        _spheres: &Vec<Sphere>,
    ) -> f64 {
        // ambient light is ambient from wherever
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

    fn compute_diffuse(&self, info: &LightComputeInfo, direction: &Vec3) -> f64 {
        // coefficient of light caught according to the angle with the light
        let coeff = direction.dot(info.normal) / (info.normal.norm() * direction.norm());
        return self.intensity * coeff;
    }

    fn compute_specular(
        &self,
        info: &LightComputeInfo,
        direction: &Vec3,
        material: &Material,
    ) -> f64 {
        return computer_specular(self.intensity, direction, info, material);
    }
}

impl Light for LightPoint {
    fn compute(&self, info: &LightComputeInfo, material: &Material, spheres: &Vec<Sphere>) -> f64 {
        // light direction
        let direction = self.position - info.position;
        if info.is_shadow {
            let opt_shadow_intersection =
                find_intersection(info.position, direction, spheres, 0.001, 1000.0);
            if opt_shadow_intersection.is_some() {
                return 0.0;
            }
        }
        if !info.is_diffuse && !info.is_specular {
            return 1.0;
        }
        let mut res = 0.0;
        if info.is_diffuse {
            res += self.compute_diffuse(info, &direction);
        }
        if info.is_specular {
            res += self.compute_specular(info, &direction, material);
        }
        return res;
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

    fn compute_diffuse(&self, info: &LightComputeInfo) -> f64 {
        // coefficient of light caught according to the angle with the light
        let coeff = self.direction.dot(info.normal) / (info.normal.norm() * self.direction.norm());
        return self.intensity * coeff;
    }

    fn compute_specular(&self, info: &LightComputeInfo, material: &Material) -> f64 {
        return computer_specular(self.intensity, &self.direction, info, material);
    }
}

impl Light for LightDirectional {
    fn compute(&self, info: &LightComputeInfo, material: &Material, spheres: &Vec<Sphere>) -> f64 {
        if info.is_shadow {
            // try to find object between the hit and the light
            let opt_shadow_intersection =
                find_intersection(info.position, self.direction, spheres, 0.001, 1000.0);
            // if so return dark
            if opt_shadow_intersection.is_some() {
                return 0.0;
            }
        }
        if !info.is_diffuse && !info.is_specular {
            return 1.0;
        }
        let mut res = 0.0;
        if info.is_diffuse {
            res += self.compute_diffuse(info);
        }
        if info.is_specular {
            res += self.compute_specular(info, material);
        }
        return res;
    }
}
