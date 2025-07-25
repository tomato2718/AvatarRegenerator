use rand::random;

pub type Gene = ((u32, u32), u32, u32, u32, [u8; 4]);

pub trait GeneEditable {
    fn get_gene(&self) -> &Gene;

    fn set_gene(&mut self, gene: Gene);
}

pub fn mutate_gene(gene: &mut Gene) {}

pub fn mix_gene(a: &Gene, b: &Gene) -> Gene {}
