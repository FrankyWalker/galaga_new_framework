use crate::settings::Settings;
use crate::ship::{new_b2_fly_ship, new_fly_ship, new_northrop_fly_ship, new_tiki_fly_ship, Ship};
use crate::structs::{Cords, COLUMNS};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

pub struct FlySpawner {
    cols: usize,
    settings: Settings,
    pub current_level: u32,
    pub flies_per_level_base: u32,
    rng: StdRng,
}

impl FlySpawner {
    pub fn new() -> Self {
        FlySpawner {
            cols: COLUMNS,
            settings: Settings::new(),
            current_level: 1,
            flies_per_level_base: 10, // Doubled from 5 to 10
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub fn spawn_flies(&mut self, fly_count: u32) -> HashMap<Cords, Box<dyn Ship>> {
        let mut grid = HashMap::new();
        // Always use wave formation regardless of level
        self.create_wave_formation(&mut grid, fly_count);
        grid
    }

    fn get_random_ship(&mut self) -> Box<dyn Ship> {
        // Randomly choose a ship type
        match self.rng.gen_range(0..4) {
            0 => new_fly_ship(),
            1 => new_tiki_fly_ship(),
            2 => new_northrop_fly_ship(),
            _ => new_b2_fly_ship(),
        }
    }

    fn create_wave_formation(&mut self, grid: &mut HashMap<Cords, Box<dyn Ship>>, fly_count: u32) {
        let mut flies_placed = 0;
        let max_rows = 5;

        // Fill the wave pattern with random ships
        for row in 0..max_rows {
            for col in (row % 3 + 2..self.cols - 2).step_by(3) {
                if flies_placed >= fly_count { return; }
                grid.insert(Cords(row, col), self.get_random_ship());
                flies_placed += 1;
            }
        }

        // If we still need more flies, fill in additional positions
        let mut row = 0;
        let mut col = 0;

        while flies_placed < fly_count {
            // Skip positions that are already filled
            if !grid.contains_key(&Cords(row, col)) {
                grid.insert(Cords(row, col), self.get_random_ship());
                flies_placed += 1;
            }

            // Move to next position
            col += 2;
            if col >= self.cols {
                col = 0;
                row = (row + 1) % max_rows;
            }
        }
    }

    pub fn spawn_next_level(&mut self) -> HashMap<Cords, Box<dyn Ship>> {
        self.current_level += 1;

        let seed = 42 + self.current_level as u64;
        self.rng = StdRng::seed_from_u64(seed);

        // Double the fly count from the original calculation
        let fly_count = (self.flies_per_level_base + self.current_level) * 2;
        self.spawn_flies(fly_count)
    }

    pub fn reset_level(&mut self) {
        self.current_level = 1;
        self.rng = StdRng::seed_from_u64(42);
    }

    pub fn get_current_fly_count(&self) -> u32 {
        // Double the fly count from the original calculation
        (self.flies_per_level_base + self.current_level) * 2
    }
}