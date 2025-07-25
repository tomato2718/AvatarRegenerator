use super::{ellipse::Ellipse, rectangle::Rectangle};
use crate::canvas::Chromosome;

#[derive(Clone, Copy)]
pub enum GaShape {
    Rect(Rectangle),
    Ellipse(Ellipse),
}

impl Chromosome for GaShape {
    fn mutate(&mut self) {
        match self {
            GaShape::Rect(rectangle) => rectangle.mutate(),
            GaShape::Ellipse(ellipse) => ellipse.mutate(),
        }
    }

    fn place(&self, image: &mut image::RgbaImage) {
        match self {
            GaShape::Rect(rectangle) => rectangle.place(image),
            GaShape::Ellipse(ellipse) => ellipse.place(image),
        }
    }
}
