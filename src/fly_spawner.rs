use crate::settings::Settings;
use crate::ship::{new_b2_fly_ship, new_fly_ship, new_northrop_fly_ship, new_tiki_fly_ship, Ship};
use crate::structs::{Cords, COLUMNS};
use rand::rngs::StdRng;
use rand::SeedableRng;
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
            flies_per_level_base: 5,
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub fn spawn_flies(&mut self, fly_count: u32) -> HashMap<Cords, Box<dyn Ship>> {
        let mut grid = HashMap::new();
        self.create_diamond_formation(&mut grid, fly_count);
        grid
    }

    fn get_ship_for_level(&self, tier: u32) -> Box<dyn Ship> {
        match tier {
            0 => {
                new_fly_ship()
            },
            1 => {
                if self.current_level >= 2 {
                    new_tiki_fly_ship()
                } else {
                    new_fly_ship()
                }
            },
            2 => {
                if self.current_level >= 3 {
                    new_northrop_fly_ship()
                } else if self.current_level >= 2 {
                    new_tiki_fly_ship()
                } else {
                    new_fly_ship()
                }
            },
            _ => {
                if self.current_level >= 4 {
                    new_b2_fly_ship()
                } else if self.current_level >= 3 {
                    new_northrop_fly_ship()
                } else if self.current_level >= 2 {
                    new_tiki_fly_ship()
                } else {
                    new_fly_ship()
                }
            }
        }
    }

    fn create_diamond_formation(&mut self, grid: &mut HashMap<Cords, Box<dyn Ship>>, fly_count: u32) {
        let middle_col = self.cols / 2;
        let mut flies_placed = 0;

        grid.insert(Cords(0, middle_col), new_tiki_fly_ship());
        flies_placed += 1;
        if flies_placed >= fly_count { return; }

        grid.insert(Cords(1, middle_col - 2), new_b2_fly_ship());
        grid.insert(Cords(1, middle_col + 2), new_b2_fly_ship());
        grid.insert(Cords(2, middle_col), new_b2_fly_ship());
        flies_placed += 3;
        if flies_placed >= fly_count { return; }

        grid.insert(Cords(2, middle_col - 4), new_northrop_fly_ship());
        grid.insert(Cords(2, middle_col + 4), new_northrop_fly_ship());
        flies_placed += 2;
        if flies_placed >= fly_count { return; }

        grid.insert(Cords(3, middle_col - 6), self.get_ship_for_level(0));
        grid.insert(Cords(3, middle_col - 2), self.get_ship_for_level(0));
        grid.insert(Cords(3, middle_col + 2), self.get_ship_for_level(0));
        grid.insert(Cords(3, middle_col + 6), self.get_ship_for_level(0));
        flies_placed += 4;
        if flies_placed >= fly_count { return; }

        let extra_positions = vec![
            (0, middle_col - 2), (0, middle_col + 2),
            (1, middle_col), (1, middle_col - 4), (1, middle_col + 4),
            (2, middle_col - 2), (2, middle_col + 2),
            (3, middle_col), (3, middle_col - 4), (3, middle_col + 4)
        ];

        for (idx, (row, col)) in extra_positions.iter().enumerate() {
            let fly_tier = idx % 4;
            grid.insert(Cords(*row, *col), self.get_ship_for_level(fly_tier as u32));
            flies_placed += 1;
            if flies_placed >= fly_count { return; }
        }
    }

    pub fn spawn_next_level(&mut self) -> HashMap<Cords, Box<dyn Ship>> {
        self.current_level += 1;


        let seed = 42 + self.current_level as u64;
        self.rng = StdRng::seed_from_u64(seed);

        let fly_count = self.flies_per_level_base + self.current_level;
        self.spawn_flies(fly_count)
    }

    pub fn reset_level(&mut self) {
        self.current_level = 1;
        self.rng = StdRng::seed_from_u64(42);
    }

    pub fn get_current_fly_count(&self) -> u32 {
        self.flies_per_level_base + self.current_level
    }
}
