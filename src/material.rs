use crate::render::RGBA;

pub struct Material {
    // color of the material
    pub color: RGBA,
    // specular i.e. the amount of light sent back by the material
    pub specular: f64,
    // reflective i.e. is the material acting as "mirror"
    pub reflective: f64,
}

impl Material {
    pub fn new(color: RGBA, specular: f64, reflective: f64) -> Self {
        return Material {
            color,
            specular,
            reflective,
        };
    }
}
