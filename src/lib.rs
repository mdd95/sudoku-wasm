use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

const GRID_SIZE: usize = 9;
const BOX_SIZE: usize = 3;
const BLANK: u8 = 0;

#[wasm_bindgen]
pub struct Sudoku {
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
    rows: [[bool; GRID_SIZE + 1]; GRID_SIZE],
    cols: [[bool; GRID_SIZE + 1]; GRID_SIZE],
    boxes: [[bool; GRID_SIZE + 1]; GRID_SIZE],
}

#[wasm_bindgen]
impl Sudoku {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sudoku {
        Sudoku {
            grid: [[BLANK; GRID_SIZE]; GRID_SIZE],
            rows: [[false; GRID_SIZE + 1]; GRID_SIZE],
            cols: [[false; GRID_SIZE + 1]; GRID_SIZE],
            boxes: [[false; GRID_SIZE + 1]; GRID_SIZE],
        }
    }

    pub fn get_grid(&self) -> Vec<u8> {
        self.grid.iter().flatten().cloned().collect()
    }

    pub fn gen(&mut self) -> bool {
        if let Some((r, c)) = self.find_blank() {
            let mut rng = thread_rng();
            let mut nums: Vec<u8> = (1..=(GRID_SIZE as u8)).collect();
            nums.shuffle(&mut rng);

            for &num in &nums {
                if self.is_safe(r, c, num) {
                    self.place_num(r, c, num, true);

                    if self.gen() {
                        return true;
                    }
                    self.place_num(r, c, num, false);
                }
            }
            false
        } else {
            true
        }
    }

    fn find_blank(&self) -> Option<(usize, usize)> {
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if self.grid[r][c] == BLANK {
                    return Some((r, c));
                }
            }
        }
        None
    }

    fn is_safe(&self, r: usize, c: usize, num: u8) -> bool {
        !self.rows[r][num as usize]
            && !self.cols[c][num as usize]
            && !self.boxes[(r / BOX_SIZE) * BOX_SIZE + (c / BOX_SIZE)][num as usize]
    }

    fn place_num(&mut self, r: usize, c: usize, num: u8, place: bool) {
        self.grid[r][c] = if place { num } else { BLANK };
        self.rows[r][num as usize] = place;
        self.cols[c][num as usize] = place;
        self.boxes[(r / BOX_SIZE) * BOX_SIZE + (c / BOX_SIZE)][num as usize] = place;
    }

    pub fn remove_num(&mut self, num_holes: u8) {
        let mut rng = thread_rng();
        let mut pos: [(usize, usize); 81] = [(0, 0); 81];
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                pos[r * GRID_SIZE + c] = (r, c);
            }
        }
        pos.shuffle(&mut rng);

        let mut holes_made: u8 = 0;

        for &(r, c) in pos.iter() {
            let removed_num = self.grid[r][c];
            self.place_num(r, c, removed_num, false);

            let mut test_sudoku = self.clone();
            if test_sudoku.is_unique_soln() {
                holes_made += 1;
                if holes_made >= num_holes {
                    break;
                }
            } else {
                self.place_num(r, c, removed_num, true);
            }
        }
    }

    fn is_unique_soln(&mut self) -> bool {
        let mut count: u8 = 0;
        self.count_soln(&mut count);
        count == 1
    }

    fn count_soln(&mut self, count: &mut u8) -> bool {
        if let Some((r, c)) = self.find_blank() {
            for num in 1..=GRID_SIZE as u8 {
                if self.is_safe(r, c, num) {
                    self.place_num(r, c, num, true);

                    if self.count_soln(count) {
                        return true;
                    }
                    self.place_num(r, c, num, false);
                }
            }
            false
        } else {
            *count += 1;
            *count > 1
        }
    }

    fn clone(&self) -> Self {
        Sudoku {
            grid: self.grid.clone(),
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            boxes: self.boxes.clone(),
        }
    }
}
