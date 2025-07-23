pub use crate::canvas::Placeable;

pub trait Shape {
    fn color(&self) -> [u8; 4];

    fn center(&self) -> (i32, i32);

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn z_index(&self) -> u32;

    fn mutate(&mut self, center: (i32, i32), width: u32, height: u32, z_index: u32, color: [u8; 4]);
}
