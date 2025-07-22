pub use crate::canvas::Placeable;

pub trait Shape {
    fn color(&self) -> [u8; 4];

    fn center(&self) -> (i32, i32);

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn z_index(&self) -> u32;
}
