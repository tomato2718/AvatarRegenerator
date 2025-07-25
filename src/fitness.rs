use ril::{Image, Rgba};

pub struct ImageFitness<'a> {
    base: &'a Image<Rgba>,
}

impl<'a> ImageFitness<'a> {
    pub fn new(base: &'a Image<Rgba>) -> Self {
        ImageFitness { base }
    }

    pub fn calculate(&self, target: &Image<Rgba>) -> u32 {
        let base_pixels = self.base.pixels();
        let target_pixels = target.pixels();
        let mut score: u32 = 0;
        for (base_row, target_row) in base_pixels.zip(target_pixels) {
            for (base_pixel, target_pixel) in base_row.iter().zip(target_row.iter()) {
                score = score.saturating_add(base_pixel.r.abs_diff(target_pixel.r) as u32);
                score = score.saturating_add(base_pixel.g.abs_diff(target_pixel.g) as u32);
                score = score.saturating_add(base_pixel.b.abs_diff(target_pixel.b) as u32);
                score = score.saturating_add(base_pixel.a.abs_diff(target_pixel.a) as u32);
            }
        }
        score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_base_and_target_should_return_fitness_score() {
        let base = Image::new(
            32,
            32,
            Rgba {
                r: 255,
                g: 0,
                b: 0,
                a: 100,
            },
        );
        let target = Image::new(
            32,
            32,
            Rgba {
                r: 0,
                g: 0,
                b: 255,
                a: 100,
            },
        );
        let fitness = ImageFitness::new(&base);

        let score = fitness.calculate(&target);

        assert_eq!(score, 32 * 32 * 255 * 2);
    }
}
