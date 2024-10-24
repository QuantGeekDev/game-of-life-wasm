mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let alert_message = format!("Hello, {}!", name);
    alert(alert_message.as_str());
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn check_underpopulation(cell: Cell, live_neighbors: u8) -> Cell {
        if cell == Cell::Alive && live_neighbors < 2 {
            Cell::Dead
        } else {
            cell
        }
    }

    fn check_survival(cell: Cell, live_neighbors: u8) -> Cell {
        if cell == Cell::Alive && (live_neighbors == 2 || live_neighbors == 3) {
            Cell::Alive
        } else {
            cell
        }
    }

    fn check_overpopulation(cell: Cell, live_neighbors: u8) -> Cell {
        if cell == Cell::Alive && live_neighbors > 3 {
            Cell::Dead
        } else {
            cell
        }
    }

    fn check_reproduction(cell: Cell, live_neighbors: u8) -> Cell {
        if cell == Cell::Dead && live_neighbors == 3 {
            Cell::Alive
        } else {
            cell
        }
    }

    fn apply_rules(cell: Cell, live_neighbors: u8) -> Cell {
        let cell = Self::check_underpopulation(cell, live_neighbors);
        let cell = Self::check_survival(cell, live_neighbors);
        let cell = Self::check_overpopulation(cell, live_neighbors);
        let cell = Self::check_reproduction(cell, live_neighbors);
        cell
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next[idx] = Universe::apply_rules(cell, live_neighbors);
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '🌚' } else { '🌞' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}