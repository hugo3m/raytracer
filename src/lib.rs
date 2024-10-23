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
    canv: render::Canvas,
    camera: Vec3,
}

#[wasm_bindgen]
impl Raytracer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Raytracer {
        Raytracer {
            canv: render::Canvas::new(width, height),
            camera: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn input(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        up: bool,
        down: bool,
    ) {
        let x: f64 = f64::from(left as u8) * -1.0 + f64::from(right as u8) * 1.0;
        let y: f64 = f64::from(down as u8) * -1.0 + f64::from(up as u8) * 1.0;
        let z: f64 = f64::from(backward as u8) * -1.0 + f64::from(forward as u8) * 1.0;
        self.camera = self.camera + Vec3::new(x * 0.1, y * 0.1, z * 0.1);
    }

    pub fn draw(&mut self) -> Vec<u8> {
        console_error_panic_hook::set_once();

        let spheres: Vec<Sphere> = vec![
            Sphere::new(
                Vec3::new(0.0, -1.0, 3.0),
                1.0,
                Material::new(RGBA::new(255, 0, 200, 255), 500.0, 0.2),
            ),
            Sphere::new(
                Vec3::new(0.0, -5001.0, 0.0),
                5000.0,
                Material::new(RGBA::new(255, 255, 0, 255), 1000.0, 0.5),
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
        ];

        let lights: Vec<Box<dyn Light>> = vec![
            Box::new(LightAmbient::new(0.2)),
            Box::new(LightPoint::new(0.6, Vec3::new(2.0, 1.0, 0.0))),
            Box::new(LightDirectional::new(0.2, Vec3::new(1.0, 4.0, 4.0))),
        ];

        for x in -self.canv.w_max..self.canv.w_max {
            for y in -self.canv.h_max + 1..self.canv.h_max {
                let viewport = self.canv.pixel_to_viewport(x, y);
                let direction = viewport - self.camera;
                self.canv.set_pixel_from_rgba(
                    x,
                    y,
                    &get_pixel_color(self.camera, direction, &spheres, &lights, 1),
                );
            }
        }

        return self.canv.render();
    }
}

fn get_pixel_color(
    origin: Vec3,
    direction: Vec3,
    spheres: &Vec<Sphere>,
    lights: &Vec<Box<dyn Light>>,
    recursion_depth: u8,
) -> RGBA {
    let opt_intersection = find_intersection(origin, direction, &spheres, 1.0, 1000.0);
    if opt_intersection.is_some() {
        let (intersection, sphere) = opt_intersection.unwrap();
        let normal = sphere.normal(intersection);
        let light_compute_info = LightComputeInfo {
            position: intersection,
            direction: direction,
            normal: normal,
        };
        let recursion_color = compute_light(&lights, &light_compute_info, &sphere, &spheres);
        if sphere.material.reflective <= 0.0 || recursion_depth <= 0 {
            return recursion_color;
        }
        let reflected = reflection(&direction, &normal);
        let reflected_color = get_pixel_color(
            intersection,
            reflected,
            spheres,
            lights,
            recursion_depth - 1,
        );
        return recursion_color * (1.0 - sphere.material.reflective)
            + reflected_color * sphere.material.reflective;
    } else {
        return RGBA::new(85, 200, 253, 255);
    }
}

fn compute_light(
    lights: &Vec<Box<dyn Light>>,
    light_compute_info: &LightComputeInfo,
    sphere: &Sphere,
    spheres: &Vec<Sphere>,
) -> RGBA {
    let mut lighting: f64 = 0.0;
    for light in lights.iter() {
        lighting += light.compute(light_compute_info, &sphere.material, spheres);
    }
    return if lighting > 0.0 {
        sphere.material.color * lighting
    } else {
        sphere.material.color * 0.0
    };
}
