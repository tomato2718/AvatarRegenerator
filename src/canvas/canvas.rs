use super::traits::Chromosome;
use image::{ImageFormat, RgbaImage};

pub struct Canvas<T: Chromosome, const S: usize> {
    width: u32,
    height: u32,
    chromosomes: [T; S],
}

impl<T: Chromosome, const S: usize> Canvas<T, S> {
    pub fn new(width: u32, height: u32, chromosomes: [T; S]) -> Self {
        Canvas {
            width,
            height,
            chromosomes,
        }
    }

    pub fn mutate(&mut self, seed: usize) {
        let i = seed % S;
        self.chromosomes[i].mutate();
    }

    pub fn crossover(&self, mate: &Canvas<T, S>) -> Self {
        let new_chromosomes: [T; S] = std::array::from_fn(|index| {
            if index & 1 == 1 {
                self.chromosomes[index].clone()
            } else {
                mate.chromosomes[index].clone()
            }
        });
        Canvas {
            width: self.width,
            height: self.height,
            chromosomes: new_chromosomes,
        }
    }

    pub fn save(&self, path: &str) {
        self.to_image()
            .save_with_format(path, ImageFormat::Png)
            .unwrap();
    }

    pub fn to_image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(self.width, self.height);
        self.chromosomes.iter().for_each(|c| c.place(&mut image));
        image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::Rgba;
    use imageproc::drawing::draw_filled_circle_mut;

    #[derive(Clone, Copy)]
    struct FakeShape {}

    impl Chromosome for FakeShape {
        fn place(&self, image: &mut RgbaImage) {
            draw_filled_circle_mut(image, (64, 64), 8, Rgba([255, 0, 0, 255]));
        }

        fn mutate(&mut self) {}
    }

    #[test]
    fn to_image_should_return_painted_rgba_image() {
        let canvas = Canvas::new(128, 128, [FakeShape {}]);

        let image = canvas.to_image();

        assert_eq!(image.get_pixel(64, 64).0, [255, 0, 0, 255]);
    }
}
