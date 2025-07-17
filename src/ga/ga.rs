use std::array::from_fn;
use std::collections::BinaryHeap;

pub struct GeneticAlgorithm<DataType: Copy> {
    crossover_rate: u8,
    crossover: fn(&DataType, &DataType) -> DataType,
    mutation_rate: u8,
    mutation: fn(&mut DataType),
    calculate_fitness: fn(&DataType) -> u8,
    random: fn() -> u8,
}

impl<DataType: Copy> GeneticAlgorithm<DataType> {
    pub fn new(
        crossover_rate: u8,
        crossover_function: fn(&DataType, &DataType) -> DataType,
        mutation_rate: u8,
        mutation_function: fn(&mut DataType),
        fitness_function: fn(&DataType) -> u8,
        random: fn() -> u8,
    ) -> Self {
        GeneticAlgorithm::<DataType> {
            crossover_rate,
            crossover: crossover_function,
            mutation_rate,
            mutation: mutation_function,
            calculate_fitness: fitness_function,
            random,
        }
    }

    pub fn execute<const S: usize>(
        &self,
        population: [DataType; S],
        elite_count: usize,
    ) -> [DataType; S] {
        let mut population = Vec::from(population);
        let mut heap = self.create_fitness_heap(&population);

        let mut elite_count = self.ensure_elite_count(elite_count);
        let mut processed = vec![];
        while elite_count > 0 {
            let (a, b) = (heap.pop().unwrap(), heap.pop().unwrap());
            let child = self
                .try_crossover(&population[a.1], &population[b.1])
                .and_then(|mut child| Some(*self.try_mutate(&mut child)));
            if let Some(c) = child {
                let score = (self.calculate_fitness)(&c) ^ 0xff;
                processed.push((score, population.len()));
                population.push(c);
            }
            processed.push(a);
            processed.push(b);
            elite_count -= 2;
        }
        processed.iter().for_each(|c| heap.push(*c));

        from_fn(|_| population[heap.pop().unwrap().1])
    }

    fn create_fitness_heap(&self, population: &[DataType]) -> BinaryHeap<(u8, usize)> {
        let mut heap = BinaryHeap::<(u8, usize)>::new();
        for (index, member) in population.iter().enumerate() {
            let score = (self.calculate_fitness)(&member) ^ 0xff;
            heap.push((score, index));
        }
        heap
    }

    fn ensure_elite_count(&self, count: usize) -> usize {
        if (count & 1) == 1 {
            return count - 1;
        }
        count
    }

    fn try_crossover(&self, a: &DataType, b: &DataType) -> Option<DataType> {
        if (self.random)() < self.crossover_rate {
            Some((self.crossover)(a, b))
        } else {
            None
        }
    }

    fn try_mutate<'a>(&self, child: &'a mut DataType) -> &'a DataType {
        if (self.random)() < self.mutation_rate {
            (self.mutation)(child);
        }
        child
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type FakeDataType = [u8; 4];

    fn fitness(data: &FakeDataType) -> u8 {
        data.iter().fold(0, |acc, x| acc + x / 4)
    }

    fn crossover(a: &FakeDataType, b: &FakeDataType) -> FakeDataType {
        let mut child = a.clone();
        child[2..].copy_from_slice(&b[2..]);
        child
    }

    fn mutate<'a>(data: &'a mut FakeDataType) {
        data.iter_mut().for_each(|v| *v ^= 0xff);
    }

    fn create_ga_instance() -> GeneticAlgorithm<FakeDataType> {
        GeneticAlgorithm::new(230, crossover, 23, mutate, fitness, || 220)
    }

    #[test]
    fn execute_given_population_should_return_evolved_generation() {
        let population: [FakeDataType; 2] = vec![
            vec![0, 0, 5, 5].try_into().unwrap(),
            vec![8, 8, 0, 0].try_into().unwrap(),
        ]
        .try_into()
        .unwrap();
        let mut expect: [FakeDataType; 2] = vec![
            vec![0, 0, 5, 5].try_into().unwrap(),
            vec![0, 0, 0, 0].try_into().unwrap(),
        ]
        .try_into()
        .unwrap();
        let ga = create_ga_instance();

        let mut new_gen = ga.execute(population, 2);

        new_gen.sort();
        expect.sort();
        assert_eq!(new_gen, expect);
    }
}
