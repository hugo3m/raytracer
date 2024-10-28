extern crate console_error_panic_hook;

pub mod geometry;
pub mod material;
pub mod math;
pub mod render;

use geometry::{
    light::{Light, LightAmbient, LightComputeInfo, LightDirectional, LightPoint},
    sphere::{find_intersection, Sphere},
};
use material::Material;
use math::vec::{reflection, Vec3};
use render::RGBA;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
struct Raytracer {
    // A canvas used to draw pixels
    canv: render::Canvas,
    // A camera represented by a position
    camera: Vec3,
    spheres: Vec<Sphere>,
    lights: Vec<Box<dyn Light>>,
    // Is diffuse light compute
    is_diffuse: bool,
    // Is specular light compute
    is_specular: bool,
    // Are shadows compute
    is_shadow: bool,
    // Is reflection compute
    is_reflection: bool,
    camera_speed: f64,
}

#[wasm_bindgen]
impl Raytracer {
    #[wasm_bindgen(constructor)]
    /// Create a sphere with the given parameters
    pub fn new(
        width: usize,
        height: usize,
        sphere_number: usize,
        is_diffuse: bool,
        is_specular: bool,
        is_shadow: bool,
        is_reflection: bool,
        camera_speed: f64,
    ) -> Raytracer {
        // main sphere
        let mut spheres = vec![
            Sphere::new(
                Vec3::new(0.0, -5001.0, 0.0),
                5000.0,
                Material::new(RGBA::new(255, 255, 0, 255), 1000.0, 0.1),
            ),
            Sphere::new(
                Vec3::new(0.0, -1.0, 3.0),
                1.0,
                Material::new(RGBA::new(255, 0, 200, 255), 500.0, 0.2),
            ),
            Sphere::new(
                Vec3::new(-2.0, 0.0, 4.0),
                1.0,
                Material::new(RGBA::new(0, 255, 0, 255), 10.0, 0.4),
            ),
            Sphere::new(
                Vec3::new(2.0, 0.0, 4.0),
                1.0,
                Material::new(RGBA::new(0, 0, 255, 255), 500.0, 0.3),
            ),
            Sphere::new(
                Vec3::new(0.0, 4.0, 10.0),
                1.0,
                Material::new(RGBA::new(120, 120, 255, 255), 500.0, 0.2),
            ),
            Sphere::new(
                Vec3::new(-3.0, 5.0, 10.0),
                1.0,
                Material::new(RGBA::new(255, 0, 255, 255), 1000.0, 0.0),
            ),
            Sphere::new(
                Vec3::new(2.0, 2.5, 15.0),
                1.0,
                Material::new(RGBA::new(0, 0, 255, 255), 500.0, 0.3),
            ),
            Sphere::new(
                Vec3::new(-1.0, 3.0, 20.0),
                1.0,
                Material::new(RGBA::new(255, 0, 200, 255), 1500.0, 0.7),
            ),
            Sphere::new(
                Vec3::new(-2.0, 1.0, 15.0),
                1.0,
                Material::new(RGBA::new(0, 255, 0, 255), 10.0, 0.4),
            ),
            Sphere::new(
                Vec3::new(0.0, 2.0, -5.0),
                1.0,
                Material::new(RGBA::new(0, 120, 255, 255), 500.0, 0.3),
            ),
        ];
        // reduce number of sphere
        spheres.truncate(sphere_number);
        Raytracer {
            canv: render::Canvas::new(width, height),
            camera: Vec3::new(0.0, 0.0, 0.75),
            spheres,
            lights: vec![
                Box::new(LightAmbient::new(0.2)),
                Box::new(LightPoint::new(0.6, Vec3::new(2.0, 1.0, 0.0))),
                Box::new(LightDirectional::new(0.2, Vec3::new(1.0, 4.0, 4.0))),
            ],
            is_diffuse,
            is_reflection,
            is_shadow,
            is_specular,
            camera_speed,
        }
    }

    // Compute the input and change camera position accordingly
    pub fn input(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        up: bool,
        down: bool,
        delta_time: f64,
    ) {
        let x: f64 = map_bool_to_f64(left) * -1.0 + map_bool_to_f64(right) * 1.0;
        let y: f64 = map_bool_to_f64(down) * -1.0 + map_bool_to_f64(up) * 1.0;
        let z: f64 = map_bool_to_f64(backward) * -1.0 + map_bool_to_f64(forward) * 1.0;
        self.camera =
            self.camera + (Vec3::new(x, y, z).normalize() * delta_time * self.camera_speed);
    }

    pub fn draw(&mut self) -> Vec<u8> {
        console_error_panic_hook::set_once();
        // for every pixels of the canvas
        for x in -self.canv.w_max..self.canv.w_max {
            for y in -self.canv.h_max + 1..self.canv.h_max {
                // retrieve viewport position in the scene
                let viewport = self.canv.pixel_to_viewport(x, y) + self.camera;
                // compute direction from camera to viewport
                let direction = (viewport - self.camera).normalize();
                // assign color
                self.canv.set_pixel_from_rgba(
                    x,
                    y,
                    &get_pixel_color(
                        self.camera,
                        direction,
                        &self.spheres,
                        &self.lights,
                        1,
                        self.is_diffuse,
                        self.is_shadow,
                        self.is_specular,
                        self.is_reflection,
                    ),
                );
            }
        }
        // return raw array of pixels
        return self.canv.render();
    }
}

/// Assign a pixel color the of the viewport
fn get_pixel_color(
    origin: Vec3,
    direction: Vec3,
    spheres: &Vec<Sphere>,
    lights: &Vec<Box<dyn Light>>,
    recursion_depth: u8,
    is_diffuse: bool,
    is_shadow: bool,
    is_specular: bool,
    is_reflection: bool,
) -> RGBA {
    // find and optional intersection
    let opt_intersection = find_intersection(origin, direction, &spheres, 1.0, 1000.0);
    // if intersected
    if opt_intersection.is_some() {
        // retrieve the information of the intersection
        let (intersection, sphere) = opt_intersection.unwrap();
        let normal = sphere.normal(intersection);
        let light_compute_info = LightComputeInfo {
            position: intersection,
            direction,
            normal,
            is_diffuse,
            is_shadow,
            is_specular,
        };
        // compute the light of the intersection
        let recursion_color = compute_light(&lights, &light_compute_info, &sphere, &spheres);
        // if not reflection just return the color
        if sphere.material.reflective <= 0.0 || recursion_depth <= 0 || !is_reflection {
            return recursion_color;
        }
        // otherwise recurse on get_pixel_color
        // by faking a camera at the position of the intersection
        // and direction of the reflection
        let reflected = reflection(&direction, &normal);
        let reflected_color = get_pixel_color(
            intersection,
            reflected,
            spheres,
            lights,
            recursion_depth - 1,
            is_diffuse,
            is_shadow,
            is_specular,
            is_reflection,
        );
        return recursion_color * (1.0 - sphere.material.reflective)
            + reflected_color * sphere.material.reflective;
    // if no intersection returns background color
    } else {
        return RGBA::new(85, 200, 253, 255);
    }
}

/// Compute the lights according to the lights in the scene and:
/// * position of the intersection in the scene
/// * direction od the ray
/// * normal of the surface
fn compute_light(
    lights: &Vec<Box<dyn Light>>,
    light_compute_info: &LightComputeInfo,
    sphere: &Sphere,
    spheres: &Vec<Sphere>,
) -> RGBA {
    let mut lighting: f64 = 0.0;
    // retrieve lighting for every single light
    for light in lights.iter() {
        lighting += light.compute(light_compute_info, &sphere.material, spheres);
    }
    return if lighting > 0.0 {
        sphere.material.color * lighting
    } else {
        sphere.material.color * 0.0
    };
}

/// Helper to map a boolean value to a f64
fn map_bool_to_f64(boolean: bool) -> f64 {
    if boolean {
        return 1.0;
    }
    return 0.0;
}
