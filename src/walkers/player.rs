extern crate async_trait;

use crate::{
    coordinates::{
        coordinate::Coordinate, default_coordinate::DefaultCoordinate,
        mutable_coordinate::MutableCoordinate,
    },
    walkers::walker::Walker,
};
use async_trait::async_trait;

pub struct Player {
    max_x: u32,
    max_y: u32,
    coordinate: MutableCoordinate,
}

impl Player {
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.coordinate
            .equals(&DefaultCoordinate::new(self.max_x, self.max_y))
    }
}

#[async_trait]
impl Walker for Player {
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
