use super::gene_edit::{Gene, GeneEditable, mix_gene, mutate_gene};
use crate::canvas::{Chromosome, Shape};
use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

pub struct Rectangle {
    gene: Gene,
}

impl Rectangle {
    pub fn new(center: (u32, u32), width: u32, height: u32, z_index: u32, color: [u8; 4]) -> Self {
        Rectangle {
            gene: (center, width, height, z_index, color),
        }
    }

    pub fn new_from_gene(gene: Gene) -> Self {
        Rectangle { gene }
    }
}

impl GeneEditable for Rectangle {
    fn get_gene(&self) -> &Gene {
        &self.gene
    }

    fn set_gene(&mut self, gene: Gene) {
        self.gene = gene;
    }
}

impl Shape for Rectangle {
    fn center(&self) -> (u32, u32) {
        self.gene.0
    }

    fn width(&self) -> u32 {
        self.gene.1
    }

    fn height(&self) -> u32 {
        self.gene.2
    }

    fn z_index(&self) -> u32 {
        self.gene.3
    }

    fn color(&self) -> [u8; 4] {
        self.gene.4
    }
}

impl Chromosome for Rectangle {
    fn place(&self, image: &mut RgbaImage) {
        let (x, y) = self.gene.0;
        let top = i32::abs(x as i32 - (self.gene.1 / 2) as i32);
        let left = i32::abs(y as i32 - (self.gene.2 / 2) as i32);
        draw_filled_rect_mut(
            image,
            Rect::at(top, left).of_size(self.gene.1, self.gene.2),
            Rgba(self.gene.4),
        );
    }

    fn mutate(&mut self) {
        mutate_gene(&mut self.gene);
    }

    fn crossover(&self, mate: &dyn Shape) -> impl Chromosome {
        let new_gene = mix_gene(
            &self.gene,
            &(
                mate.center(),
                mate.width(),
                mate.height(),
                mate.z_index(),
                mate.color(),
            ),
        );
        Rectangle::new_from_gene(new_gene)
    }
}
