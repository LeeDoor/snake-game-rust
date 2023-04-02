mod grid;
use std::io;
use std::io::Write;

fn main() {
    let grid = grid::generate_grid();
    grid::show_grid(&grid);
}
