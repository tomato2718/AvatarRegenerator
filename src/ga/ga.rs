use super::Cacheable;
use std::collections::BinaryHeap;

pub struct GeneticAlgorithm<'a, Individual: Cacheable> {
    crossover_rate: u8,
    crossover: fn(&Individual, &Individual) -> Individual,
    mutation_rate: u8,
    mutation: fn(&mut Individual),
    fitness: &'a dyn Fn(&Individual) -> u32,
    random: fn() -> u8,
}

impl<'a, Individual: Cacheable> GeneticAlgorithm<'a, Individual> {
    pub fn new(
        crossover_rate: u8,
        crossover_function: fn(&Individual, &Individual) -> Individual,
        mutation_rate: u8,
        mutation_function: fn(&mut Individual),
        fitness_function: &'a dyn Fn(&Individual) -> u32,
        random: fn() -> u8,
    ) -> Self {
        GeneticAlgorithm::<Individual> {
            crossover_rate,
            crossover: crossover_function,
            mutation_rate,
            mutation: mutation_function,
            fitness: fitness_function,
            random,
        }
    }

    pub fn execute(&self, mut population: Vec<Individual>, elite_count: usize) -> Vec<Individual> {
        let size = population.len();
        let mut heap = self.create_fitness_heap(&mut population);

        let mut elite_count = self.ensure_elite_count(elite_count);
        let mut processed = vec![];
        while elite_count > 0 {
            let (a, b) = (heap.pop().unwrap(), heap.pop().unwrap());
            let child = self
                .try_crossover(&population[a.1], &population[b.1])
                .and_then(|mut child| {
                    self.try_mutate(&mut child);
                    Some(child)
                });
            if let Some(mut c) = child {
                let score = self.calculate_fitness(&mut c);
                processed.push((score, population.len()));
                population.push(c);
            }
            processed.push(a);
            processed.push(b);
            elite_count -= 2;
        }
        processed.iter().for_each(|c| heap.push(*c));

        let mut to_remove: Vec<usize> = (0..size).map(|_| heap.pop().unwrap().1).collect();
        to_remove.sort_by(|a, b| b.cmp(a));
        (0..size)
            .map(|i| population.swap_remove(to_remove[i]))
            .collect()
    }

    fn create_fitness_heap(&self, population: &mut Vec<Individual>) -> BinaryHeap<(u32, usize)> {
        let mut heap = BinaryHeap::<(u32, usize)>::new();
        for (index, member) in population.iter_mut().enumerate() {
            let score = self.calculate_fitness(member);
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

    fn try_crossover(&self, a: &Individual, b: &Individual) -> Option<Individual> {
        if (self.random)() < self.crossover_rate {
            Some((self.crossover)(a, b))
        } else {
            None
        }
    }

    fn try_mutate<'b>(&self, child: &'b mut Individual) -> &'b Individual {
        if (self.random)() < self.mutation_rate {
            (self.mutation)(child);
        }
        child
    }

    fn calculate_fitness(&self, individual: &mut Individual) -> u32 {
        match individual.get_fitness() {
            Some(score) => *score,
            None => {
                let score = !(self.fitness)(&individual);
                individual.set_fitness(score);
                score
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy)]
    struct FakeDataType(pub [u8; 4]);

    impl Cacheable for FakeDataType {
        fn get_fitness(&self) -> &Option<u32> {
            &None
        }

        fn set_fitness(&mut self, fitness: u32) {}
    }

    fn fitness(data: &FakeDataType) -> u32 {
        data.0.iter().fold(0, |acc, x| acc + x) as u32
    }

    fn crossover(a: &FakeDataType, b: &FakeDataType) -> FakeDataType {
        let mut child = a.clone();
        child.0[2..].copy_from_slice(&b.0[2..]);
        child
    }

    fn mutate<'a>(data: &'a mut FakeDataType) {
        data.0.iter_mut().for_each(|v| *v ^= 0xff);
    }

    fn create_ga_instance() -> GeneticAlgorithm<'static, FakeDataType> {
        GeneticAlgorithm::new(230, crossover, 23, mutate, &fitness, || 220)
    }

    #[test]
    fn execute_given_population_should_return_evolved_generation() {
        let population = vec![
            FakeDataType([5, 5, 0, 0]),
            FakeDataType([0, 0, 3, 3]),
            FakeDataType([4, 5, 6, 7]),
            FakeDataType([5, 6, 7, 8]),
        ];

        let mut expect = vec![
            FakeDataType([0, 0, 0, 0]),
            FakeDataType([0, 0, 3, 3]),
            FakeDataType([5, 5, 0, 0]),
            FakeDataType([4, 5, 6, 7]),
        ];
        let ga = create_ga_instance();

        let mut new_gen = ga.execute(population, 2);

        new_gen.sort_by(|a, b| a.0.cmp(&b.0));
        expect.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            new_gen.iter().map(|c| c.0).collect::<Vec<[u8; 4]>>(),
            expect.iter().map(|c| c.0).collect::<Vec<[u8; 4]>>()
        );
    }
}
