use super::traits::Placeable;
use image::{ColorType, DynamicImage, GenericImageView, ImageFormat};

pub struct Canvas {
    image: DynamicImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            image: DynamicImage::new(width, height, ColorType::Rgba8),
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        self.image.get_pixel(x, y).0
    }

    pub fn draw(&mut self, shape: &dyn Placeable) {
        shape.place(&mut self.image);
    }

    pub fn save(&self, path: &str) {
        self.image.save_with_format(path, ImageFormat::Png).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::Rgba;
    use imageproc::drawing::draw_filled_circle_mut;

    struct FakeShape {}

    impl Placeable for FakeShape {
        fn place(&self, image: &mut DynamicImage) {
            draw_filled_circle_mut(image, (64, 64), 8, Rgba([255, 0, 0, 100]));
        }
    }

    #[test]
    fn draw_given_shape_draw_on_canvas() {
        let mut canvas = Canvas::new(128, 128);
        let shape = FakeShape {};

        canvas.draw(&shape);

        assert_eq!(canvas.get_pixel(64, 64), [255, 0, 0, 100]);
    }
}
