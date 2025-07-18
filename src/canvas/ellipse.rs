use super::traits::{Placeable, Shape};
use image::Rgba;
use imageproc::drawing::draw_filled_ellipse_mut;

pub struct Ellipse {
    center: (i32, i32),
    width: u32,
    height: u32,
    z_index: u32,
    color: [u8; 4],
}

impl Ellipse {
    pub fn new(center: (i32, i32), width: u32, height: u32, z_index: u32, color: [u8; 4]) -> Self {
        Ellipse {
            center,
            width,
            height,
            z_index,
            color,
        }
    }
}

impl Shape for Ellipse {
    fn color(&self) -> [u8; 4] {
        self.color
    }

    fn center(&self) -> (i32, i32) {
        self.center
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn z_index(&self) -> u32 {
        self.z_index
    }
}

impl Placeable for Ellipse {
    fn place(&self, image: &mut image::DynamicImage) {
        draw_filled_ellipse_mut(
            image,
            self.center,
            (self.width / 2).try_into().unwrap(),
            (self.height / 2).try_into().unwrap(),
            Rgba(self.color),
        );
    }
}
