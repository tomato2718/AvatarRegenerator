use image::DynamicImage;

pub trait Shape {
    fn crossover(shape: &impl Shape) -> Self;

    fn color(&self) -> [u8; 4];

    fn center(&self) -> [u32; 2];

    fn z_index(&self) -> u32;

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

pub trait Placeable {
    fn place(&self, image: &mut DynamicImage);
}
