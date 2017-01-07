use direction::Direction;
use point::Point;

#[derive(Clone, Debug)]
pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(head: Point, num_segments: u8, direction: Direction) -> Snake {
        let mut body = Vec::with_capacity(num_segments as usize);
        let mut cur = head;
        let dir_point = direction.as_point();
        for _ in 0..num_segments {
            cur = cur - dir_point;
            body.push(cur);
        }
        Snake {
            head: head,
            body: body,
            direction: direction
        }
    }
    pub fn move_forward(&mut self) {
        self.body.insert(0, self.head);
        let end = self.body.len() - 1;
        self.body.remove(end);
        self.head = self.head + self.direction.as_point()
    }
    pub fn move_forward_and_eat(&mut self) {
        self.body.insert(0, self.head);
        self.head = self.head + self.direction.as_point()
    }
    pub fn next_position(&self) -> Point {
        self.head + self.direction.as_point()
    }
}
