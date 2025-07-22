use super::{ellipse::Ellipse, rectangle::Rectangle, traits::GaShape};
use rand::random;

const shape_size: u32 = 0x7f;

pub fn random_shape(boundary: (u32, u32)) -> Box<dyn GaShape> {
    if (random::<u8>() & 0x1) == 1 {
        Box::new(random_ellipse(boundary))
    } else {
        Box::new(random_rect(boundary))
    }
}

fn random_ellipse(boundary: (u32, u32)) -> Ellipse {
    Ellipse::new(
        (
            (random::<u32>() % boundary.0) as i32,
            (random::<u32>() % boundary.1) as i32,
        ),
        (random::<u32>() & shape_size) + 1,
        (random::<u32>() & shape_size) + 1,
        random(),
        [random(), random(), random(), random()],
    )
}

fn random_rect(boundary: (u32, u32)) -> Rectangle {
    Rectangle::new(
        (
            (random::<u32>() % boundary.0) as i32,
            (random::<u32>() % boundary.1) as i32,
        ),
        (random::<u32>() & shape_size) + 1,
        (random::<u32>() & shape_size) + 1,
        random(),
        [random(), random(), random(), random()],
    )
}
