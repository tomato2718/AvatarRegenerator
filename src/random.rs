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

pub fn crossover(a: &GaShape, b: &GaShape) -> GaShape {
    let seed = random::<u16>();

    let center = (
        if seed & 0b1 != 0 {
            a.center().0
        } else {
            b.center().0
        },
        if seed & 0b10 != 0 {
            a.center().1
        } else {
            b.center().1
        },
    );
    let width = if seed & 0b100 != 0 {
        a.width()
    } else {
        b.width()
    };
    let height = if seed & 0b1000 != 0 {
        a.height()
    } else {
        b.height()
    };
    let z_index = if seed & 0b10000 != 0 {
        a.z_index()
    } else {
        b.z_index()
    };
    let color = [
        if seed & 0b100000 != 0 {
            a.color()[0]
        } else {
            b.color()[0]
        },
        if seed & 0b1000000 != 0 {
            a.color()[1]
        } else {
            b.color()[1]
        },
        if seed & 0b10000000 != 0 {
            a.color()[2]
        } else {
            b.color()[2]
        },
        if seed & 0b100000000 != 0 {
            a.color()[3]
        } else {
            b.color()[3]
        },
    ];
    match a {
        GaShape::Ellipse(_) => {
            GaShape::Ellipse(Ellipse::new(center, width, height, z_index, color))
        }
        GaShape::Rect(_) => GaShape::Rect(Rectangle::new(center, width, height, z_index, color)),
    }
}

// TODO: To finish
pub fn mutate(shape: &mut GaShape) {}

pub fn random_u8() -> u8 {
    random()
}
