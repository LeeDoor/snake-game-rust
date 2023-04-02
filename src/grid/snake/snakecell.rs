use crate::grid::point::Point;

pub struct SnakeCell {
    pos: Point
}

impl SnakeCell {
    pub fn new(pos_: Point) -> SnakeCell {
        SnakeCell {pos: pos_}
    }

    pub fn get_pos(&self) -> &Point {
        &self.pos
    }
}