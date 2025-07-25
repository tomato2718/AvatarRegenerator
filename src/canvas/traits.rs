use image::RgbaImage;

pub trait Chromosome: Copy {
    fn place(&self, image: &mut RgbaImage);

    fn mutate(&mut self);
}
