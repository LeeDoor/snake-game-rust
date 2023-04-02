pub mod direction;
use direction::Direction;

use super::point::Point;

mod snakecell;
use snakecell::SnakeCell;
use std::mem;

const START_DIR: Direction = Direction::Left;
const START_SNAKE_LENGTH: usize = 5;
const START_HEAD_POSITION: Point = Point{x: 5, y: 5};


pub struct Snake {
    pub body: Vec<SnakeCell>,
    pub direction: Direction
}

impl Snake {
    pub fn new(maxlen: usize) -> Snake {
        let mut snake = Snake{
            body: Vec::with_capacity(maxlen), 
            direction: START_DIR};

        for i in 0..START_SNAKE_LENGTH {
            snake.body.push(SnakeCell::new(Point{x: START_HEAD_POSITION.x - i as i32, y: START_HEAD_POSITION.y }, Direction::Right));
        }
        return snake;
    }

    pub fn head(&self) -> &SnakeCell {
        return self.body.last().unwrap();
    }

    pub fn walk(&mut self, direction: Direction) {
        let mut cur_dir: Direction = direction;
        self.body[0] = self.body[0] + cur_dir.to_point();
        cur_dir = self.body[0].dir;
        self.body[0].dir = direction;

        for i in 1..self.body.len() {
            mem::swap(&mut self.body[i].dir, &mut cur_dir);
            self.body[i] = self.body[i] + self.body[i].dir.to_point();
        }
    }
}