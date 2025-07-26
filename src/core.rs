use crate::canvas::Canvas;
use crate::fitness::ImageFitness;
use crate::ga::GeneticAlgorithm;
use crate::random::{random_shape, random_u8, random_u32};
use crate::shape::GaShape;
use ril::{Image, Rgba};

pub struct Arguments {
    pub target: String,
    pub pupulation: usize,
    pub elite: usize,
    pub chromosome: usize,
    pub crossover_rate: u8,
    pub mutation_rate: u8,
    pub iterations: usize,
    pub shape_max_size: u32,
    pub output_dir: String,
    pub save_every: usize,
}

pub fn run(args: Arguments) {
    let image = Image::<Rgba>::open(args.target).unwrap();
    let fitness = ImageFitness::new(&image);
    let fitness_wrapper = |c: &Canvas<GaShape>| fitness.calculate(&c.to_image());
    let ga = GeneticAlgorithm::<Canvas<GaShape>>::new(
        args.crossover_rate,
        |a, b| a.crossover(b),
        args.mutation_rate,
        |c| c.mutate(random_u32() as usize),
        &fitness_wrapper,
        random_u8,
    );
    let mut population: Vec<Canvas<GaShape>> = (0..args.pupulation)
        .map(|_| {
            Canvas::new(
                image.width(),
                image.height(),
                (0..args.chromosome)
                    .map(|_| {
                        random_shape(
                            (image.width(), image.height()),
                            (args.shape_max_size, args.shape_max_size),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    for i in 1..=args.iterations {
        population = ga.execute(population, args.elite);
        if i % args.save_every == 0 {
            population[0].save(format!("{}/{:04}.png", args.output_dir, i).as_str());
            println!(
                "Generation {}: Done!, Score: {}",
                i,
                fitness_wrapper(&population[0])
            );
        }
    }
}
