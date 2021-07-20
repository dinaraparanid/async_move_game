pub trait Coordinate {
    fn get_x(&self) -> u32;
    fn get_y(&self) -> u32;

    fn new(x: u32, y: u32) -> Self
    where
        Self: Sized;

    #[inline]
    fn equals(&self, other: &impl Coordinate) -> bool
    where
        Self: Sized,
    {
        self.get_x() == other.get_x() && self.get_y() == other.get_y()
    }
}
