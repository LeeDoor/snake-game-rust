use super::super::point::Point;

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Top,
    Left,
    Bottom
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Right => Point {x: 1, y: 0},
            Direction::Top => Point {x: 0, y: -1},
            Direction::Left => Point {x: -1, y: 0},
            Direction::Bottom => Point {x: 0, y: 1}
        }
    }
}