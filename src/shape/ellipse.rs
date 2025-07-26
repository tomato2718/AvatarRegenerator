use super::gene::Gene;
use ril::{Image, Rgba};
use std::cmp::max;

#[derive(Clone, Copy)]
pub struct Ellipse {
    gene: Gene,
}

impl Ellipse {
    pub fn new(gene: Gene) -> Self {
        Ellipse { gene }
    }

    pub fn place(&self, image: &mut Image<Rgba>) {
        let boundary = (max(self.gene.width / 2, 1), max(self.gene.height / 2, 1));
        let shape = ril::Ellipse::new()
            .with_position(
                max(self.gene.center.0, boundary.0),
                max(self.gene.center.1, boundary.1),
            )
            .with_radii(boundary.0, boundary.1)
            .with_fill(Rgba::new(
                self.gene.color[0],
                self.gene.color[1],
                self.gene.color[2],
                self.gene.color[3],
            ));
        image.draw(&shape);
    }

    pub fn mutate(&mut self) {
        self.gene.mutate();
    }
}
