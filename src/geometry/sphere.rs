use crate::material::Material;
use crate::math::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    /// Creates a new sphere
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            material,
            radius,
        }
    }
    /// Compute an optional intersection
    /// between the ray of:
    /// * origin
    /// * direction
    /// and the sphere
    pub fn intersect(
        &self,
        origin: Vec3,
        direction: Vec3,
        distance_min: f64,
        distance_max: f64,
    ) -> Option<Vec3> {
        let co: Vec3 = origin - self.center;
        // quadratic solution
        let a: f64 = direction.dot(direction);
        let b: f64 = 2.0 * direction.dot(co);
        let c: f64 = co.dot(co) - self.radius * self.radius;
        let discriminant = (b * b) - (4.0 * a * c);
        // if only one solution
        if discriminant == 0.0 {
            let t0 = -b / (2.0 * a);
            if t0 > distance_min && t0 < distance_max {
                return Some(origin + direction * t0);
            }
        }
        // if two solutions
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            // find the smallest one fitting in the intervals
            if t1 > distance_min
                && t1 < distance_max
                && (t1 < t2 || t2 < distance_min || t2 > distance_max)
            {
                return Some(origin + direction * t1);
            }
            if t2 > distance_min
                && t2 < distance_max
                && (t2 < t1 || t1 < distance_min || t1 > distance_max)
            {
                return Some(origin + direction * t2);
            }
        }
        return None;
    }

    /// Get the normal vector of the intersection
    pub fn normal(&self, intersection: Vec3) -> Vec3 {
        return intersection - self.center;
    }
}

/// Find an optional intersection in the scenes between:
/// * the ray
/// * all the spheres
pub fn find_intersection(
    origin: Vec3,
    direction: Vec3,
    spheres: &Vec<Sphere>,
    distance_min: f64,
    distance_max: f64,
) -> Option<(Vec3, &Sphere)> {
    let mut opt_result: Option<(Vec3, &Sphere)> = None;
    let mut opt_closest_distance: Option<f64> = None;
    // for every spheres
    for sphere in spheres.iter() {
        // try to find a optional intersection with current sphere
        let opt_current_intersection =
            sphere.intersect(origin, direction, distance_min, distance_max);
        // if interesection
        if let Some(intersection) = opt_current_intersection {
            let distance = (intersection - origin).norm();
            // keep if it is closer
            if opt_closest_distance.is_none() || distance < opt_closest_distance.unwrap() {
                opt_closest_distance = Some(distance);
                opt_result = Some((intersection, sphere));
            }
        }
    }
    return opt_result;
}
