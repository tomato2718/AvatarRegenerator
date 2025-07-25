use super::gene::Gene;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_ellipse_mut;

#[derive(Clone, Copy)]
pub struct Ellipse {
    gene: Gene,
}

impl Ellipse {
    pub fn new(gene: Gene) -> Self {
        Ellipse { gene }
    }

    pub fn place(&self, image: &mut RgbaImage) {
        draw_filled_ellipse_mut(
            image,
            (self.gene.center.0 as i32, self.gene.center.1 as i32),
            (self.gene.width / 2) as i32,
            (self.gene.height / 2) as i32,
            Rgba(self.gene.color),
        );
    }

    pub fn mutate(&mut self) {
        self.gene.mutate();
    }
}
