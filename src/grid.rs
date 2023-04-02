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
        for col in &self.val{
            for cell in col {
                print!("{} ", celltype::to_string(cell));
            }
            print!("\n");
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
            let curPos = body.getPos();
            self.val[curPos.x][curPos.y] = celltype::CellType::Snake;
        }
    }
}

