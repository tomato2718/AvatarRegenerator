use ril::{Image, Rgba};

pub trait Chromosome: Copy {
    fn place(&self, image: &mut Image<Rgba>);

    fn mutate(&mut self);
}
