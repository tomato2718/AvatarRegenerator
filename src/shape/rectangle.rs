use super::gene::Gene;
use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

#[derive(Clone, Copy)]
pub struct Rectangle {
    gene: Gene,
}

impl Rectangle {
    pub fn new(gene: Gene) -> Self {
        Rectangle { gene }
    }

    pub fn place(&self, image: &mut RgbaImage) {
        let (x, y) = self.gene.center;
        let top = i32::abs(x as i32 - (self.gene.width / 2) as i32);
        let left = i32::abs(y as i32 - (self.gene.height / 2) as i32);
        draw_filled_rect_mut(
            image,
            Rect::at(top, left).of_size(self.gene.width, self.gene.height),
            Rgba(self.gene.color),
        );
    }

    pub fn mutate(&mut self) {
        self.gene.mutate();
    }
}
