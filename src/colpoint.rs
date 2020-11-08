use super::conf::*;

pub struct ColPoint {
    pub point: Point,
    pub color: Color,
}

impl std::cmp::PartialEq<Point> for ColPoint {
    fn eq(&self, other: &Point) -> bool {
        self.point == *other
    }
}

impl std::cmp::PartialEq for ColPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
