// https://dinesh-gdk.medium.com/game-of-life-in-rust-bb3f5ea32c09

use rand::Rng;
use std::clone::Clone;
use std::default::Default;
use std::fmt::Display;
use std::marker::Copy;
use std::{fmt, thread, time};

fn main() {
    let mut board = Board::new(10, 10, 30);
    loop {
        if board.count_alive_cells() == 0 {
            break;
        }
        println!("{}", board);
        println!(
            "Generation {} : {} alive cells",
            board.generation,
            board.count_alive_cells()
        );
        board.next_generation();
        sleep(1000);
        clear_console();
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    generation: usize,
}

impl Board {
    fn new(rows: usize, columns: usize, randomized_pourcentage: usize) -> Board {
        let mut board = Board {
            cells: vec![vec![Cell::default(); columns]; rows],
            generation: 1,
        };
        board.randomize_cells(randomized_pourcentage);
        board
    }

    fn randomize_cells(&mut self, pourcentage: usize) {
        assert!(pourcentage <= 100);
        let row_count = self.count_rows();
        let column_count = self.count_columns();
        let randomized_cells_count =
            ((row_count * column_count) as f32 * (pourcentage as f32 / 100.0)) as u32;
        let mut rng = rand::thread_rng();
        for _ in 0..randomized_cells_count {
            let x = rng.gen_range(0..row_count);
            let y = rng.gen_range(0..column_count);
            self.cells[x][y].alive = true;
        }
    }

    fn next_generation(&mut self) {
        let mut cells_copy = self.cells.to_vec();
        for x in 0..cells_copy.len() {
            for y in 0..cells_copy[x].len() {
                let neighbours = self.count_alive_neighbours(x, y);
                if neighbours < 2 || neighbours > 3 {
                    cells_copy[x][y].alive = false;
                } else if neighbours == 3 {
                    cells_copy[x][y].alive = true;
                }
            }
        }
        self.generation += 1;
        self.cells = cells_copy;
    }

    fn count_alive_neighbours(&self, x: usize, y: usize) -> usize {
        let mut neighbours = vec![];
        if x > 0 {
            if let Some(lower_row) = self.cells.get(x - 1) {
                if y > 0 {
                    neighbours.push(lower_row.get(y - 1));
                }
                if y < self.count_columns() - 1 {
                    neighbours.push(lower_row.get(y + 1));
                }
                neighbours.push(lower_row.get(y));
            }
        }
        if let Some(row) = self.cells.get(x) {
            if y > 0 {
                neighbours.push(row.get(y - 1));
            }
            if y < self.count_columns() - 1 {
                neighbours.push(row.get(y + 1));
            }
        }
        if x < self.count_rows() - 1 {
            if let Some(upper_row) = self.cells.get(x + 1) {
                if y > 0 {
                    neighbours.push(upper_row.get(y - 1));
                }
                if y < self.count_columns() - 1 {
                    neighbours.push(upper_row.get(y + 1));
                }
                neighbours.push(upper_row.get(y));
            }
        }
        neighbours.iter().fold(0, |acc, cell| match cell {
            Some(cell) => {
                if cell.alive {
                    acc + 1
                } else {
                    acc
                }
            }
            _ => acc,
        })
    }

    fn count_rows(&self) -> usize {
        self.cells.len()
    }

    fn count_columns(&self) -> usize {
        if self.cells.is_empty() {
            return 0;
        }
        self.cells[0].len()
    }

    fn count_alive_cells(&self) -> usize {
        self.cells.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc2, cell| if cell.alive { acc2 + 1 } else { acc2 })
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column_lenght = self.count_columns();
        let line_separator = (0..=column_lenght).map(|_| "- ").collect::<String>();
        write!(f, "{}\n", line_separator).unwrap();
        for row in self.cells.iter() {
            write!(f, "|").unwrap();
            for cell in row.iter() {
                match cell.alive {
                    false => write!(f, " ").unwrap(),
                    true => write!(f, "x").unwrap(),
                };
                write!(f, "|").unwrap();
            }
            write!(f, "\n{}\n", line_separator).unwrap();
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
struct Cell {
    alive: bool,
}

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}
