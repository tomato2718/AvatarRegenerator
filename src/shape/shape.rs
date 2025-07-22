use super::traits::{Placeable, Shape};
use super::{ellipse::Ellipse, rectangle::Rectangle};

#[derive(Clone, Copy)]
pub enum GaShape {
    Rect(Rectangle),
    Ellipse(Ellipse),
}

impl Placeable for GaShape {
    fn place(&self, image: &mut image::DynamicImage) {
        match self {
            GaShape::Rect(rectangle) => rectangle.place(image),
            GaShape::Ellipse(ellipse) => ellipse.place(image),
        }
    }
}

impl Shape for GaShape {
    fn center(&self) -> (i32, i32) {
        match self {
            GaShape::Rect(rectangle) => rectangle.center(),
            GaShape::Ellipse(ellipse) => ellipse.center(),
        }
    }

    fn color(&self) -> [u8; 4] {
        match self {
            GaShape::Rect(rectangle) => rectangle.color(),
            GaShape::Ellipse(ellipse) => ellipse.color(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            GaShape::Rect(rectangle) => rectangle.height(),
            GaShape::Ellipse(ellipse) => ellipse.height(),
        }
    }

    fn width(&self) -> u32 {
        match self {
            GaShape::Rect(rectangle) => rectangle.width(),
            GaShape::Ellipse(ellipse) => ellipse.width(),
        }
    }

    fn z_index(&self) -> u32 {
        match self {
            GaShape::Rect(rectangle) => rectangle.z_index(),
            GaShape::Ellipse(ellipse) => ellipse.z_index(),
        }
    }
}
