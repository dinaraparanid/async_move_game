extern crate async_trait;

use crate::{
    coordinates::{coordinate::Coordinate, mutable_coordinate::MutableCoordinate},
    walkers::walker::Walker,
};
use async_trait::async_trait;

pub struct DefaultWalker {
    max_x: u32,
    max_y: u32,
    coordinate: MutableCoordinate,
}

#[async_trait]
impl Walker for DefaultWalker {
    #[inline]
    fn get_max_x(&self) -> u32 {
        self.max_x
    }

    #[inline]
    fn get_max_y(&self) -> u32 {
        self.max_y
    }

    #[inline]
    fn get_coordinate(&self) -> &MutableCoordinate {
        &self.coordinate
    }

    #[inline]
    fn get_coordinate_mut(&mut self) -> &mut MutableCoordinate {
        &mut self.coordinate
    }

    #[inline]
    fn new(x: u32, y: u32, max_x: u32, max_y: u32) -> Self {
        Self {
            max_x,
            max_y,
            coordinate: MutableCoordinate::new(x, y),
        }
    }
}
