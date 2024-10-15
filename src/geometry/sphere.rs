use crate::math::vec::Vec3;
use crate::render::RGBA;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: RGBA,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: RGBA) -> Self {
        Sphere {
            center,
            color,
            radius,
        }
    }

    pub fn intersect(&self, origin: Vec3, direction: Vec3) -> Option<Vec3> {
        let co: Vec3 = origin - self.center;
        let a: f64 = direction.dot(direction);
        let b: f64 = 2.0 * direction.dot(co);
        let c: f64 = co.dot(co) - self.radius * self.radius;
        let discriminant = (b * b) - (4.0 * a * c);
        let mut t = -1.0;
        if discriminant == 0.0 {
            let t0 = -b / (2.0 * a);
            if t0 > 0.0 {
                t = t0
            }
        }
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t1 > 0.0 && (t1 < t2 || t2 < 0.0) {
                t = t1
            }
            if t2 > 0.0 && (t2 < t1 || t1 < 0.0) {
                t = t2
            }
        }
        if t > 0.0 {
            return Some(origin + direction * t);
        }
        return None;
    }

    pub fn normal(&self, intersection: Vec3) -> Vec3 {
        return intersection - self.center;
    }
}
