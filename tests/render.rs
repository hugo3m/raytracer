#[cfg(test)]
mod tests {
    use wasm::render::{Canvas, RGBA};

    const WIDTH: usize = 400;
    const HEIGHT: usize = 600;

    #[test]
    fn test_canvas_new() {
        let canv = Canvas::new(WIDTH, HEIGHT);
        assert_eq!(canv.width, WIDTH);
        assert_eq!(canv.height, HEIGHT);
        assert_eq!(canv.w_max, 200);
    }

    #[test]
    fn test_canvas_get_index() {
        let canv = Canvas::new(WIDTH, HEIGHT);
        assert_eq!(
            canv.get_pixel_flat_index(0, 0),
            (WIDTH * HEIGHT / 2) + WIDTH / 2
        );
    }

    #[test]
    fn test_rgba_add() {
        let rgba_1 = RGBA::new(0, 10, 120, 255);
        let rgba_2 = RGBA::new(120, 60, 120, 253);
        let rgba_3 = rgba_1 + rgba_2;
        assert_eq!(rgba_3.r, 120);
        assert_eq!(rgba_3.g, 70);
        assert_eq!(rgba_3.b, 240);
        assert_eq!(rgba_3.a, 255);
    }
}
