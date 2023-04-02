mod grid;
use grid::snake::direction::Direction;

fn main() {
    let mut a = grid::Grid::new();
    a.show_grid();
    a.snake.walk(Direction::Top);
    a.snake.walk(Direction::Top);
    a.snake.walk(Direction::Right);
    a.snake.walk(Direction::Top);
    a.show_grid();

}
