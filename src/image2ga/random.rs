use crate::shape::{Ellipse, GaShape, Rectangle, Shape};
use rand::random;

const SHAPE_SIZE: u32 = 0x7f;

pub fn random_shape(boundary: (u32, u32)) -> GaShape {
    let center = (
        (random::<u32>() % boundary.0) as i32,
        (random::<u32>() % boundary.1) as i32,
    );
    let width = (random::<u32>() & SHAPE_SIZE) + 1;
    let height = (random::<u32>() & SHAPE_SIZE) + 1;
    let z_index = random::<u32>();
    let color: [u8; 4] = [random(), random(), random(), random()];

    if (random::<u8>() & 0x1) == 1 {
        GaShape::Ellipse(Ellipse::new(center, width, height, z_index, color))
    } else {
        GaShape::Rect(Rectangle::new(center, width, height, z_index, color))
    }
}

pub fn crossover(shape_a: &GaShape, shape_b: &GaShape) -> GaShape {
    let seed = random::<u16>();
    let (mut x, mut y) = shape_a.center();
    let mut width = shape_a.width();
    let mut height = shape_a.height();
    let mut z_index = shape_a.z_index();
    let [mut r, mut g, mut b, mut a] = shape_a.color();

    if seed & 0b1 == 1 {
        x = shape_b.center().0;
    }
    if seed & 0b10 == 1 {
        y = shape_b.center().1;
    }
    if seed & 0b100 == 1 {
        width = shape_b.width();
    }
    if seed & 0b1000 == 1 {
        height = shape_b.height();
    }
    if seed & 0b10000 == 1 {
        z_index = shape_b.z_index();
    }
    if seed & 0b100000 == 1 {
        r = shape_b.color()[0];
    }
    if seed & 0b1000000 == 1 {
        g = shape_b.color()[1];
    }
    if seed & 0b10000000 == 1 {
        b = shape_b.color()[2];
    }
    if seed & 0b100000000 == 1 {
        a = shape_b.color()[3];
    }
    match shape_a {
        GaShape::Ellipse(_) => {
            GaShape::Ellipse(Ellipse::new((x, y), width, height, z_index, [r, g, b, a]))
        }
        GaShape::Rect(_) => {
            GaShape::Rect(Rectangle::new((x, y), width, height, z_index, [r, g, b, a]))
        }
    }
}

pub fn mutate(shape: &mut GaShape) {
    let seed = random::<u16>();
    let (mut x, mut y) = shape.center();
    let mut width = shape.width();
    let mut height = shape.height();
    let mut z_index = shape.z_index();
    let [mut r, mut g, mut b, mut a] = shape.color();

    if seed & 0b1 == 1 {
        x = !x;
    }
    if seed & 0b10 == 1 {
        y = !y;
    }
    if seed & 0b100 == 1 {
        width = !width;
    }
    if seed & 0b1000 == 1 {
        height = !height;
    }
    if seed & 0b10000 == 1 {
        z_index = !z_index;
    }
    if seed & 0b100000 == 1 {
        r = !r;
    }
    if seed & 0b1000000 == 1 {
        g = !g;
    }
    if seed & 0b10000000 == 1 {
        b = !b;
    }
    if seed & 0b100000000 == 1 {
        a = !a;
    }
    shape.mutate((x, y), width, height, z_index, [r, g, b, a]);
}

pub fn random_u8() -> u8 {
    random()
}
