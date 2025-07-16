pub struct GeneticAlgorithm<T> {
    crossover_rate: u8,
    mutation_rate: u8,
    fitness: fn() -> u8,
    crossover: fn(T, T) -> T,
}

impl<T> GeneticAlgorithm<T> {
    pub fn new(
        crossover_rate: u8,
        mutation_rate: u8,
        fitness: fn() -> u8,
        crossover: fn(T, T) -> T,
    ) -> Self {
        GeneticAlgorithm::<T> {
            crossover_rate,
            mutation_rate,
            fitness,
            crossover,
        }
    }

    pub fn execute(&self) -> Vec<T> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn execute_given_population_should_return_evolved_chromosome() {}
}
