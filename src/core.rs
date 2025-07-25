use crate::canvas::Canvas;
use crate::fitness::ImageFitness;
use crate::ga::GeneticAlgorithm;
use crate::random::{random_shape, random_u8, random_u32};
use crate::shape::GaShape;
use ril::{Image, Rgba};
use std::array;

const POPULATION_SIZE: usize = 20;
const ELITE_COUNT: usize = 18;
const CHROMOSOME_COUNT: usize = 100;
const CROSSOVER_RATE: u8 = 220;
const MUTATION_RATE: u8 = 20;
const ITERATIONS: usize = 5000;
const MAX_SHAPE_SIZE: u32 = 64;

pub fn run() {
    let image = Image::<Rgba>::open("./sample/rickroll.jpg").unwrap();
    let fitness = ImageFitness::new(&image);
    let fitness_wrapper = |c: &Canvas<GaShape, CHROMOSOME_COUNT>| fitness.calculate(&c.to_image());
    let ga = GeneticAlgorithm::<Canvas<GaShape, CHROMOSOME_COUNT>>::new(
        CROSSOVER_RATE,
        |a, b| a.crossover(b),
        MUTATION_RATE,
        |c| c.mutate(random_u32() as usize),
        &fitness_wrapper,
        random_u8,
    );
    let mut population: [Canvas<GaShape, CHROMOSOME_COUNT>; POPULATION_SIZE] =
        array::from_fn(|_| {
            Canvas::new(
                image.width(),
                image.height(),
                array::from_fn(|_| {
                    random_shape(
                        (image.width(), image.height()),
                        (MAX_SHAPE_SIZE, MAX_SHAPE_SIZE),
                    )
                }),
            )
        });

    for i in 1..=ITERATIONS {
        population = ga.execute(population, ELITE_COUNT);
        if i % 50 == 0 {
            population[0].save(format!("./output/{:03}.png", i).as_str());
            println!(
                "Generation {}: Done!, Score: {}",
                i,
                fitness_wrapper(&population[0])
            );
        }
    }
}
