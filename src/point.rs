use rand;
use rand::Rng;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    /// Constructs a new Point from the x and y values
    pub fn new(x: i16, y: i16) -> Point {
        Point { x: x, y: y}
    }
    /// Creates a new point with random x and y values in a certain range.
    pub fn random(min_x: i16, max_x: i16, min_y: i16, max_y: i16) -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(min_x, max_x);
        let y = rng.gen_range(min_y, max_y);
        Point::new(x, y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}
