use crate::grid::point::Point;
use std::ops;

use super::direction::Direction;

#[derive(Copy, Clone)]
pub struct SnakeCell {
    pos: Point,
    pub dir: Direction
}

impl SnakeCell {
    pub fn new(pos_: Point, dir_: Direction) -> SnakeCell {
        SnakeCell {pos: pos_, dir: dir_}
    }

    pub fn get_pos(&self) -> &Point {
        &self.pos
    }
}

impl ops::Add<Point> for SnakeCell {
    type Output = SnakeCell;
    fn add (self, sec: Point) ->SnakeCell {
        SnakeCell {
            pos: self.pos + sec,
            dir: self.dir
        }
    }
}

impl<'a, 'b> ops::Add<&'a Point> for &'b SnakeCell {
    type Output = SnakeCell;
    fn add (self, sec: &Point) ->Self::Output {
        SnakeCell {
            pos: self.pos + *sec,
            dir: self.dir
        }
    }
}
