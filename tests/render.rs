#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use wasm::render::Canvas;

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
}
