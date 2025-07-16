use clap::Parser;

#[derive(Parser)]
#[command(version, about = None, long_about = None)]
pub struct Args {
    #[arg(value_name = "PATH", help = "Target image")]
    pub target: String,

    #[arg(short, long, value_name = "PATH", help = "Output directory")]
    pub output_dir: String,

    #[arg(short, long, value_name = "INT", help = "Iterations")]
    pub iterations: u32,
}

pub fn parse_args() -> Args {
    Args::parse()
}
