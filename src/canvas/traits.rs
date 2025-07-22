use image::DynamicImage;

pub trait Placeable {
    fn place(&self, image: &mut DynamicImage);
}
