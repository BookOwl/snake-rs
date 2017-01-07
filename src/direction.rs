use point;
use rand::{Rand, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    // I'm gonna move my feet tonight
    // LIKE NOBODY'S WATCHING! :P
}

impl Direction {
    pub fn as_point(&self) -> point::Point {
        match *self {
            Direction::Up    => point::Point::new(0, 1),
            Direction::Down  => point::Point::new(0, -1),
            Direction::Left  => point::Point::new(-1, 0),
            Direction::Right => point::Point::new(1, 0),
        }
    }
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Direction {
        match rng.gen_range(0, 3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(), // The return value of gen_range will only be 0, 1, 2, or 3
        }
    }
}
