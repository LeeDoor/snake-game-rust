mod celltype;
mod snake;
use snake::Snake;

type RowType = Vec<celltype::CellType>;
type GridType = Vec<RowType>;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

pub struct Grid {
    val: GridType,
    snake: Snake
}

impl Grid{
    pub fn new () ->Grid {
        Grid{
            val: Grid::generate_grid(),
            snake: Snake::new(WIDTH * HEIGHT)
        }
    }
    
    pub fn show_grid(&mut self) {
        self.apply_snake();

        for y in 0..self.val[0].len() {
            let mut line = String::with_capacity(WIDTH);
            for x in 0..self.val.len(){
                line.push(celltype::to_string(&self.val[x][y]));
                line.push(' ');
            }
            print!("{}\n", line);
        }
    }

    fn generate_grid() ->GridType {
        let mut grid = GridType::with_capacity(WIDTH);
        for i in 0..WIDTH{
            grid.push(RowType::with_capacity(HEIGHT));
            for _j in 0..HEIGHT {
                grid[i].push(celltype::CellType::Ground);
            }
        }
        return grid;
    }

    fn apply_snake(&mut self) {
        for body in &self.snake.body {
            let cur_pos = body.getPos();
            self.val[cur_pos.x][cur_pos.y] = celltype::CellType::Snake;
        }
    }
}

