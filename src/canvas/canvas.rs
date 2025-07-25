use super::traits::Chromosome;
use crate::ga::Cacheable;
use ril::{Image, ImageFormat, OverlayMode, Rgba};

pub struct Canvas<T: Chromosome, const S: usize> {
    width: u32,
    height: u32,
    chromosomes: [T; S],
    fitness: Option<u32>,
}

impl<T: Chromosome, const S: usize> Cacheable for Canvas<T, S> {
    fn get_fitness(&self) -> &Option<u32> {
        &self.fitness
    }

    fn set_fitness(&mut self, fitness: u32) {
        self.fitness = Some(fitness);
    }
}

impl<T: Chromosome, const S: usize> Canvas<T, S> {
    pub fn new(width: u32, height: u32, chromosomes: [T; S]) -> Self {
        Canvas {
            width,
            height,
            chromosomes,
            fitness: None,
        }
    }

    pub fn mutate(&mut self, seed: usize) {
        let i = seed % S;
        self.chromosomes[i].mutate();
        self.fitness = None;
    }

    pub fn crossover(&self, mate: &Canvas<T, S>) -> Self {
        let new_chromosomes: [T; S] = std::array::from_fn(|index| {
            if index & 1 == 1 {
                self.chromosomes[index].clone()
            } else {
                mate.chromosomes[index].clone()
            }
        });
        Canvas::new(self.width, self.height, new_chromosomes)
    }

    pub fn save(&self, path: &str) {
        self.to_image().save(ImageFormat::Png, path).unwrap();
    }

    pub fn to_image(&self) -> Image<Rgba> {
        let mut image = Image::<Rgba>::new(
            self.width,
            self.height,
            Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        )
        .with_overlay_mode(OverlayMode::Merge);
        self.chromosomes.iter().for_each(|c| c.place(&mut image));
        image
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ril::{Image, Rgba, draw::Ellipse};

    #[derive(Clone, Copy)]
    struct FakeShape {}

    impl Chromosome for FakeShape {
        fn place(&self, image: &mut Image<Rgba>) {
            let shape = Ellipse::new()
                .with_position(64, 64)
                .with_radii(8, 8)
                .with_fill(Rgba::new(255, 0, 0, 255));
            image.draw(&shape);
        }

        fn mutate(&mut self) {}
    }

    #[test]
    fn to_image_should_return_painted_rgba_image() {
        let canvas = Canvas::new(128, 128, [FakeShape {}]);

        let image = canvas.to_image();

        assert_eq!(image.get_pixel(64, 64).unwrap(), &Rgba::new(255, 0, 0, 255));
    }
}
