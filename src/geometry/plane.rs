use crate::math::vec::Vec3;
pub struct Plane {
    pub normal: Vec3,
    pub d: f64,
}

impl Plane {
    pub fn new(normal: Vec3, d: f64) -> Self {
        Plane { normal, d }
    }
}
