pub trait Cacheable {
    fn get_fitness(&self) -> &Option<u32>;

    fn set_fitness(&mut self, fitness: u32);
}
