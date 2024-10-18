use crate::render::RGBA;

pub struct Material {
    pub color: RGBA,
    pub specular: f64,
}

impl Material {
    pub fn new(color: RGBA, specular: f64) -> Self {
        return Material { color, specular };
    }
}
