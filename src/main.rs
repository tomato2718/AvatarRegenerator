use avaregen::{Arguments, run};

use clap::{Parser};

#[derive(Parser)]
#[command(version, about = None, long_about = None)]
pub struct CLIArgs {
    #[arg(value_name = "PATH", help = "Target image")]
    pub target: String,

    #[arg(long, value_name = "INT", help = "Population size")]
    pub population: usize,

    #[arg(
        long,
        value_name = "INT",
        help = "Elite to pick for each iteration, MUST be smaller than population_size"
    )]
    pub elite: usize,

    #[arg(
        long,
        value_name = "INT",
        help = "Count of chromosome of each individual"
    )]
    pub chromosome: usize,

    #[arg(long, value_name = "INT", help = "Crossover rate, 0~255")]
    pub crossover: u8,

    #[arg(long, value_name = "INT", help = "Mutation rate, 0~255")]
    pub mutation: u8,

    #[arg(short, long, value_name = "PATH", help = "Output directory")]
    pub output_dir: String,

    #[arg(
        long,
        value_name = "INT",
        help = "Max width and height of shape"
    )]
    pub shape: u32,

    #[arg(long, value_name = "INT", help = "Save every N iterations")]
    pub save: usize,

    #[arg(long, value_name = "INT", help = "Iterations")]
    pub iterations: usize,

    
}

fn main() {
    let args = CLIArgs::parse();
    run(Arguments {
        target: args.target,
        pupulation: args.population,
        elite: args.elite,
        chromosome: args.chromosome,
        crossover_rate: args.crossover,
        mutation_rate: args.mutation,
        iterations: args.iterations,
        shape_max_size: args.shape,
        output_dir: args.output_dir,
        save_every: args.save,
    });
}
