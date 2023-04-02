mod direction;
use direction::Direction;

mod point;
use point::Point;

const START_DIR: Direction = Direction::Left;
const START_SNAKE_LENGTH: usize = 5;
const START_HEAD_POSITION: Point = Point{x: 5, y: 5};

pub struct SnakeCell {
    pos: Point
}

pub struct Snake {
    pub body: Vec<SnakeCell>,
    pub direction: Direction
}

impl SnakeCell {
    pub fn getPos(&self) -> &Point {
        &self.pos
    }
}

impl Snake {
    pub fn new(maxlen: usize) -> Snake {
        let mut snake = Snake{
            body: Vec::with_capacity(maxlen), 
            direction: START_DIR};

        for i in 0..START_SNAKE_LENGTH {
            snake.body.push(SnakeCell{ pos: Point{ x: START_HEAD_POSITION.x - i, y: START_HEAD_POSITION.y }});
        }
        return snake;
    }

    pub fn head(&self) -> &SnakeCell {
        return self.body.last().unwrap();
    }
}