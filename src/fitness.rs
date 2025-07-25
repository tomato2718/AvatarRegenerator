use image::RgbaImage;

pub struct ImageFitness<'a> {
    base: &'a RgbaImage,
}

impl<'a> ImageFitness<'a> {
    pub fn new(base: &'a RgbaImage) -> Self {
        ImageFitness { base }
    }

    pub fn calculate(&self, target: &RgbaImage) -> u32 {
        let base_pixels = self.base.pixels();
        let target_pixels = target.pixels();
        let mut score = 0;
        for (bp, tp) in base_pixels.zip(target_pixels) {
            score +=
                bp.0.iter()
                    .zip(tp.0.iter())
                    .map(|(a, b)| (*a as i32 - *b as i32).abs() as u32)
                    .sum::<u32>();
        }
        score / (target.width() * target.height() / 16)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::Rgba;

    #[test]
    fn given_base_and_target_should_return_fitness_score() {
        let base = RgbaImage::from_pixel(32, 32, Rgba([255, 0, 0, 100]));
        let target = RgbaImage::from_pixel(32, 32, Rgba([0, 255, 0, 100]));
        let fitness = ImageFitness::new(&base);

        let score = fitness.calculate(&target);

        assert_eq!(score, (255 + 255) * 32 * 32);
    }
}
