pub mod celltype;
pub mod snake;
pub mod point;
use snake::Snake;

type RowType = Vec<celltype::CellType>;
type GridType = Vec<RowType>;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct Grid {
    val: GridType,
    pub snake: Snake
}

impl Grid{
    pub fn new () ->Grid {
        Grid{
            val: Grid::generate_grid(),
            snake: Snake::new(WIDTH * HEIGHT)
        }
    }
    
    pub fn show_grid(&mut self) {
        self.update();

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

    fn update(&mut self) {
        self.val = Grid::generate_grid();
        self.apply_snake();

    }

    fn apply_snake(&mut self) {
        for body in &self.snake.body {
            let cur_pos = body.get_pos();
            self.val[cur_pos.x as usize][cur_pos.y as usize] = celltype::CellType::Snake;
        }
    }
}

