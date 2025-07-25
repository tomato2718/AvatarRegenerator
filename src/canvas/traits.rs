use image::RgbaImage;

pub trait Shape {
    fn color(&self) -> [u8; 4];

    fn center(&self) -> (u32, u32);

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn z_index(&self) -> u32;
}

pub trait Chromosome: Shape + Copy {
    fn place(&self, image: &mut RgbaImage);

    fn mutate(&mut self);
}
