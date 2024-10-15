extern crate console_error_panic_hook;

pub mod geometry;
pub mod math;
pub mod render;

use geometry::{
    light::{LightAmbient, LightDirectional, LightPoint},
    sphere::Sphere,
};
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
        Sphere::new(Vec3::new(0.0, -1.0, 3.0), 1.0, RGBA::new(255, 0, 200, 255)),
        Sphere::new(
            Vec3::new(0.0, -5001.0, 0.0),
            5000.0,
            RGBA::new(255, 255, 0, 255),
        ),
        Sphere::new(Vec3::new(-10.0, 5.0, 50.0), 1.0, RGBA::new(0, 255, 0, 255)),
    ];

    for x in -canv.w_max..canv.w_max {
        for y in -canv.h_max + 1..canv.h_max {
            let viewport = canv.pixel_to_viewport(x, y);
            let direction = viewport - cam;
            let opt_intersection = find_intersection(cam, direction, &spheres);
            if opt_intersection.is_some() {
                let (intersection, sphere) = opt_intersection.unwrap();
                let normal = sphere.normal(intersection);
                let color = sphere.color * compute_light(intersection, normal);
                canv.set_pixel_from_rgba(x, y, &color);
            } else {
                canv.set_pixel(x, y, 255, 255, 255, 255);
            }
        }
    }

    return canv.render();
}

fn find_intersection(
    origin: Vec3,
    direction: Vec3,
    spheres: &Vec<Sphere>,
) -> Option<(Vec3, &Sphere)> {
    let mut opt_result: Option<(Vec3, &Sphere)> = None;
    let mut opt_closest_distance: Option<f64> = None;
    for sphere in spheres.iter() {
        let opt_current_intersection = sphere.intersect(origin, direction);
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

fn compute_light(point: Vec3, normal: Vec3) -> f64 {
    let ambient_light = LightAmbient::new(0.2);
    let point_light = LightPoint::new(0.6, Vec3::new(-2.0, 1.0, 0.0));
    let directional_light = LightDirectional::new(0.2, Vec3::new(1.0, 4.0, 4.0));
    return ambient_light.compute()
        + point_light.compute(point, normal)
        + directional_light.compute(normal);
}
