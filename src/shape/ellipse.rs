use super::gene_edit::{Gene, GeneEditable, mix_gene, mutate_gene};
use crate::canvas::{Chromosome, Shape};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_ellipse_mut;

pub struct Ellipse {
    gene: Gene,
}

impl Ellipse {
    pub fn new(center: (u32, u32), width: u32, height: u32, z_index: u32, color: [u8; 4]) -> Self {
        Ellipse {
            gene: (center, width, height, z_index, color),
        }
    }

    pub fn new_from_gene(gene: Gene) -> Self {
        Ellipse { gene }
    }
}

impl GeneEditable for Ellipse {
    fn get_gene(&self) -> &Gene {
        &self.gene
    }

    fn set_gene(&mut self, gene: Gene) {
        self.gene = gene;
    }
}

impl Shape for Ellipse {
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

impl Chromosome for Ellipse {
    fn place(&self, image: &mut RgbaImage) {
        draw_filled_ellipse_mut(
            image,
            (self.gene.0.0 as i32, self.gene.0.1 as i32),
            (self.gene.1 / 2) as i32,
            (self.gene.2 / 2) as i32,
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
        Ellipse::new_from_gene(new_gene)
    }
}
