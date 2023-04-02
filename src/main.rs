mod grid;

fn main() {
    let grid = grid::generate_grid();
    grid::show_grid(&grid);
}
