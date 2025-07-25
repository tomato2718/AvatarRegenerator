use crate::shape::{Ellipse, GaShape, Gene, Rectangle};
use rand::random;

pub fn random_shape(boundary: (u32, u32), maxsize: (u32, u32)) -> GaShape {
    let center = (random::<u32>() % boundary.0, random::<u32>() % boundary.1);
    let width = (random::<u32>() % maxsize.0) + 1;
    let height = (random::<u32>() % maxsize.1) + 1;
    let color: [u8; 4] = [random(), random(), random(), random()];
    let gene = Gene {
        boundary,
        max_width: maxsize.0,
        mex_height: maxsize.1,
        center,
        width,
        height,
        color,
    };

    if (random::<u8>() & 0x1) == 1 {
        GaShape::Ellipse(Ellipse::new(gene))
    } else {
        GaShape::Rect(Rectangle::new(gene))
    }
}

pub fn random_u8() -> u8 {
    random()
}

pub fn random_u32() -> u32 {
    random()
}
