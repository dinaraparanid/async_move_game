use crate::coordinates::coordinate::Coordinate;

#[derive(Copy, Clone, Debug)]
pub struct MutableCoordinate {
    x: u32,
    y: u32,
}

impl MutableCoordinate {
    #[inline]
    pub(crate) fn up(&mut self) {
        self.y -= 1
    }

    #[inline]
    pub(crate) fn down(&mut self) {
        self.y += 1
    }

    #[inline]
    pub(crate) fn left(&mut self) {
        self.x -= 1
    }

    #[inline]
    pub(crate) fn right(&mut self) {
        self.x += 1
    }
}

impl Coordinate for MutableCoordinate {
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
