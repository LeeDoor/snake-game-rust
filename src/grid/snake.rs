mod direction;
use direction::Direction;

mod point;
use point::Point;

const START_DIR: Direction = Direction::Left;
const START_SNAKE_LENGTH: u32 = 5;
const START_HEAD_POSITION: Point = Point{x: 5, y: 5};

struct SnakeCell {
    pos: Point
}

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
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