use crate::material::Material;
use crate::math::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            material,
            radius,
        }
    }

    pub fn intersect(
        &self,
        origin: Vec3,
        direction: Vec3,
        distance_min: f64,
        distance_max: f64,
    ) -> Option<Vec3> {
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
            if t1 > distance_min
                && t1 < distance_max
                && (t1 < t2 || t2 < distance_min || t2 > distance_max)
            {
                t = t1
            }
            if t2 > distance_min
                && t2 < distance_max
                && (t2 < t1 || t1 < distance_min || t1 > distance_max)
            {
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

pub fn find_intersection(
    origin: Vec3,
    direction: Vec3,
    spheres: &Vec<Sphere>,
    distance_min: f64,
    distance_max: f64,
) -> Option<(Vec3, &Sphere)> {
    let mut opt_result: Option<(Vec3, &Sphere)> = None;
    let mut opt_closest_distance: Option<f64> = None;
    for sphere in spheres.iter() {
        let opt_current_intersection =
            sphere.intersect(origin, direction, distance_min, distance_max);
        if let Some(intersection) = opt_current_intersection {
            let distance = (intersection - origin).norm();
            if opt_closest_distance.is_none() || distance < opt_closest_distance.unwrap() {
                opt_closest_distance = Some(distance);
                opt_result = Some((intersection, sphere));
            }
        }
    }
    return opt_result;
}
