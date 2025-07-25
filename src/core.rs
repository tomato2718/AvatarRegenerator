use crate::args::parse_args;
use crate::canvas::Canvas;
use crate::ga::GeneticAlgorithm;
use crate::image2ga::{ImageFitness, crossover, mutate, random_shape, random_u8};
use crate::shape::{GaShape, Shape};
use image::{RgbaImage, open};

pub fn run() {
    // let _ = parse_args();
    let image = open("./sample/rickroll.jpg").unwrap().to_rgba8();
    let mut shapes = init_shapes::<1000>((image.width(), image.height()));
    let full_fit = ImageFitness::new(&image);
    let mut canvas = Canvas::new(image.width(), image.height());
    shapes.iter().for_each(|shape| {
        canvas.draw(shape);
    });
    for i in 0..=1000 {
        let fit = fitness_wrapper(&image, &canvas);
        let ga = GeneticAlgorithm::new(200, crossover, 25, mutate, &fit, random_u8);
        shapes = ga.execute(shapes, 8);
        shapes.sort_by(|a, b| a.z_index().cmp(&b.z_index()));

        let mut canvas = Canvas::new(image.width(), image.height());
        if i % 10 == 0 {
            shapes.iter().for_each(|shape| {
                canvas.draw(shape);
            });
            canvas.save(format!("./output/{:03}.png", i).as_str());
            println!(
                "Generation {:03} Done, score: {}",
                i,
                full_fit.calculate(&canvas.to_rgba8())
            );
        }
    }
}

fn init_shapes<const S: usize>(boundary: (u32, u32)) -> [GaShape; S] {
    std::array::from_fn(|_| random_shape(boundary))
}

fn fitness_wrapper(base_image: &RgbaImage, current_canvas: &Canvas) -> impl Fn(&GaShape) -> u32 {
    return |new_shape| {
        let mut c = current_canvas.clone();
        let fitness = ImageFitness::new(base_image);
        c.draw(new_shape);
        fitness.calculate(&c.to_rgba8())
    };
}
