use std::ops;

use crate::math::vec::Vec3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }

    pub fn unpack(self) -> [u8; 4] {
        return [self.r, self.g, self.b, self.a];
    }
}

impl ops::Mul<f64> for RGBA {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        let mut r_f = f64::from(self.r) * _rhs;
        let mut g_f = f64::from(self.g) * _rhs;
        let mut b_f = f64::from(self.b) * _rhs;
        if r_f > 255.0 {
            r_f = 255.0;
        }
        if r_f < 0.0 {
            r_f = 0.0;
        }
        if g_f > 255.0 {
            g_f = 255.0;
        }
        if g_f < 0.0 {
            g_f = 0.0;
        }
        if b_f > 255.0 {
            b_f = 255.0;
        }
        if b_f < 0.0 {
            b_f = 0.0;
        }
        return Self::new(r_f as u8, g_f as u8, b_f as u8, self.a);
    }
}

impl ops::Add<Self> for RGBA {
    type Output = Self;

    fn add(self, _rhs: RGBA) -> Self {
        return Self::new(
            self.r.saturating_add(_rhs.r),
            self.g.saturating_add(_rhs.g),
            self.b.saturating_add(_rhs.b),
            self.a.saturating_add(_rhs.a),
        );
    }
}

/// A 2D Canvas
#[derive(Debug, Clone)]
pub struct Canvas {
    pub height: usize,
    pub h_max: isize,
    pub pixels: Vec<RGBA>,
    viewport: Viewport,
    pub width: usize,
    pub w_max: isize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            height,
            h_max: (height / 2) as isize,
            width,
            w_max: (width / 2) as isize,
            pixels: vec![RGBA::new(0, 0, 0, 0); width * height],
            viewport: Viewport::new(1, 1, 1),
        }
    }

    pub fn get_pixel_flat_index(&self, x: isize, y: isize) -> usize {
        // from (0,0 center) to (0,0 top)
        let x_index = x + self.w_max;
        let y_index = self.h_max - y;
        // from (0,0 top) to flat array
        let index: usize = ((y_index * self.w_max * 2) + x_index) as usize;
        return index;
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, red: u8, green: u8, blue: u8, alpha: u8) {
        let index = self.get_pixel_flat_index(x, y);
        self.pixels[index] = RGBA::new(red, green, blue, alpha);
    }

    pub fn set_pixel_from_rgba(&mut self, x: isize, y: isize, rgba: &RGBA) {
        self.set_pixel(x, y, rgba.r, rgba.g, rgba.b, rgba.a)
    }

    pub fn render(&self) -> Vec<u8> {
        let map: Vec<[u8; 4]> = self
            .pixels
            .clone()
            .into_iter()
            .map(|rgba: RGBA| rgba.unpack())
            .collect();
        let res: Vec<u8> = map.iter().flatten().cloned().collect();
        return res;
    }

    pub fn pixel_to_viewport(&self, x: isize, y: isize) -> Vec3 {
        // to float
        let x_f: f64 = (x as i32).into();
        let y_f: f64 = (y as i32).into();
        let viewport_width: f64 = (self.viewport.width as u32).into();
        let viewport_height: f64 = (self.viewport.height as u32).into();
        let width: f64 = (self.width as u32).into();
        let height: f64 = (self.height as u32).into();
        // calculate ratio
        let width_ratio = viewport_width / width;
        let height_ratio = viewport_height / height;
        // project to viewport
        let res_x = x_f * width_ratio;
        let res_y = y_f * height_ratio;
        Vec3::new(res_x, res_y, (self.viewport.depth as u32).into())
    }
}

// A 2D viewport in a 3D environment
#[derive(Debug, Clone)]
pub struct Viewport {
    pub depth: usize,
    pub height: usize,
    pub width: usize,
}

impl Viewport {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Viewport {
            width,
            height,
            depth,
        }
    }
}
