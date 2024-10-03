use std::process::Command;

use grid::Grid;

pub mod grid;

fn main() {
    let mut grid = Grid::new(15, 15);

    grid.seed();
    loop {
        Command::new("clear").status().unwrap();
        print!("{}", grid);
        grid.update();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
