use crate::coordinates::coordinate::Coordinate;

#[derive(Copy, Clone, Debug)]
pub struct DefaultCoordinate {
    x: u32,
    y: u32,
}

impl Coordinate for DefaultCoordinate {
    #[inline]
    fn get_x(&self) -> u32 {
        self.x
    }

    #[inline]
    fn get_y(&self) -> u32 {
        self.y
    }

    #[inline]
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
