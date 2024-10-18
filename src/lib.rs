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
use math::vec::Vec3;
use render::RGBA;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn draw(width: usize, height: usize) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let mut canv = render::Canvas::new(width, height);
    let cam = Vec3::new(0.0, 0.0, 0.0);

    let spheres: Vec<Sphere> = vec![
        Sphere::new(
            Vec3::new(0.0, -1.0, 3.0),
            1.0,
            Material::new(RGBA::new(255, 0, 200, 255), 500.0),
        ),
        Sphere::new(
            Vec3::new(0.0, -5001.0, 0.0),
            5000.0,
            Material::new(RGBA::new(255, 255, 0, 255), 1000.0),
        ),
        Sphere::new(
            Vec3::new(-2.0, 0.0, 4.0),
            1.0,
            Material::new(RGBA::new(0, 255, 0, 255), 10.0),
        ),
        Sphere::new(
            Vec3::new(2.0, 0.0, 4.0),
            1.0,
            Material::new(RGBA::new(0, 0, 255, 255), 500.0),
        ),
    ];

    let lights: Vec<Box<dyn Light>> = vec![
        Box::new(LightAmbient::new(0.2)),
        Box::new(LightPoint::new(0.6, Vec3::new(2.0, 1.0, 0.0))),
        Box::new(LightDirectional::new(0.2, Vec3::new(1.0, 4.0, 4.0))),
    ];

    for x in -canv.w_max..canv.w_max {
        for y in -canv.h_max + 1..canv.h_max {
            let viewport = canv.pixel_to_viewport(x, y);
            let direction = viewport - cam;
            let opt_intersection = find_intersection(cam, direction, &spheres, 1.0, 1000.0);
            if opt_intersection.is_some() {
                let (intersection, sphere) = opt_intersection.unwrap();
                let normal = sphere.normal(intersection);
                let light_compute_info = LightComputeInfo {
                    position: intersection,
                    direction: direction,
                    normal: normal,
                };
                let color = compute_light(&lights, &light_compute_info, &sphere, &spheres);
                canv.set_pixel_from_rgba(x, y, &color);
            } else {
                canv.set_pixel(x, y, 255, 255, 255, 255);
            }
        }
    }

    return canv.render();
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
