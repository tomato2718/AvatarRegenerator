use super::{ellipse::Ellipse, rectangle::Rectangle, traits::PlaceableShape};
use rand::random;

pub fn random_shape(boundary: (u32, u32)) -> Box<dyn PlaceableShape> {
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
        random::<u32>() & 0x3f,
        random::<u32>() & 0x3f,
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
        random::<u32>() & 0x3f,
        random::<u32>() & 0x3f,
        random(),
        [random(), random(), random(), random()],
    )
}
