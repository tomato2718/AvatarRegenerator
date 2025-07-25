use super::gene::Gene;
use ril::{Image, Rgba};

#[derive(Clone, Copy)]
pub struct Rectangle {
    gene: Gene,
}

impl Rectangle {
    pub fn new(gene: Gene) -> Self {
        Rectangle { gene }
    }

    pub fn place(&self, image: &mut Image<Rgba>) {
        let (x, y) = self.gene.center;
        let top = x.saturating_sub(self.gene.width / 2);
        let left = y.saturating_sub(self.gene.height / 2);
        let shape = ril::Rectangle::at(top, left)
            .with_size(self.gene.width, self.gene.height)
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
