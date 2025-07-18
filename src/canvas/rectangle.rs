use super::traits::{Placeable, Shape};
use image::Rgba;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

pub struct Rectangle {
    center: (i32, i32),
    width: u32,
    height: u32,
    z_index: u32,
    color: [u8; 4],
}

impl Rectangle {
    pub fn new(center: (i32, i32), width: u32, height: u32, z_index: u32, color: [u8; 4]) -> Self {
        Rectangle {
            center,
            width,
            height,
            z_index,
            color,
        }
    }
}

impl Shape for Rectangle {
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

impl Placeable for Rectangle {
    fn place(&self, image: &mut image::DynamicImage) {
        let (x, y) = self.center;
        let top = x - i32::abs(self.height as i32) / 2;
        let left = y - i32::abs(self.width as i32) / 2;
        draw_filled_rect_mut(
            image,
            Rect::at(top, left).of_size(self.width, self.height),
            Rgba(self.color),
        );
    }
}
