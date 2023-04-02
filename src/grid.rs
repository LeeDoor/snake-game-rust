mod celltype;
mod snake;

type RowType = Vec<celltype::CellType>;
type GridType = Vec<RowType>;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;

pub fn generate_grid() ->GridType {
    let mut grid = GridType::with_capacity(WIDTH);
    for i in 0..WIDTH{
        grid.push(RowType::with_capacity(HEIGHT));
        for _j in 0..HEIGHT {
            grid[i].push(celltype::CellType::Ground);
        }
    }
    return grid;
}

pub fn show_grid(grid: &GridType) {
    for col in grid{
        for cell in col {
            print!("{} ", celltype::to_string(cell));
        }
        print!("\n");
    }
}