use point;
pub struct Apple {
    pub position: point::Point,
}
impl Apple {
    pub fn new(position: point::Point) -> Apple {
        Apple { position: position }
    }
}
