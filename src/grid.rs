use std::fmt;

use rand::Rng;

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,  // 0 = dead, 1 = alive
    buffer: Vec<Vec<u8>>, // used to avoid cells.clone() in cicle
    alive_symbol: char,
    dead_symbol: char,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cells = vec![vec![0; width]; height];
        let buffer = vec![vec![0; width]; height];
        Grid {
            width,
            height,
            cells,
            buffer,
            alive_symbol: '👽',
            dead_symbol: '🌑',
        }
    }

    pub fn seed(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut rng = rand::thread_rng();
                let random_value: u8 = rng.gen_range(0..=1);
                match random_value {
                    value if value == 1 => self.set_alive(x, y),
                    _ => (),
                }
            }
        }
    }

    pub fn set_alive_symbol(&mut self, c: char) {
        self.alive_symbol = c;
    }
    pub fn set_dead_symbol(&mut self, c: char) {
        self.dead_symbol = c;
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y][x] = 1;
        }
    }

    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nx = (x as isize + i) as usize;
                let ny = (y as isize + j) as usize;
                if nx < self.width && ny < self.height {
                    count += self.cells[ny][nx];
                }
            }
        }
        count
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                let is_alive = self.cells[y][x] == 1;

                self.buffer[y][x] = match (is_alive, alive_neighbors) {
                    // can continue to live if it has 2 or 3 living neighbours
                    (true, 2) | (true, 3) => 1,
                    // dead cell born if it has 3 living neighbours
                    (false, 3) => 1,
                    // default cell is dead
                    _ => 0,
                };
            }
        }
        //merge update from buffer to cells
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for &cell in row {
                let symbol = if cell == 1 {
                    self.alive_symbol
                } else {
                    self.dead_symbol
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
