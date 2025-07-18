use image::DynamicImage;

pub trait Shape {
    fn color(&self) -> [u8; 4];

    fn center(&self) -> (i32, i32);

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn z_index(&self) -> u32;
}

pub trait Placeable {
    fn place(&self, image: &mut DynamicImage);
}
