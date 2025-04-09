use std::collections::HashMap;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS};
use crate::settings::Settings;

pub struct FlySpawner {
    cols: usize,
    settings: Settings
}

impl FlySpawner {
    pub fn new() -> Self {
        FlySpawner {
            cols: COLUMNS,
            settings: Settings::new()
        }
    }

    pub fn spawn_flies(&self, fly_count: u32) -> HashMap<Cords, Ship> {
        let spacing = if fly_count == 0 { 1 } else { self.cols as u32 / (fly_count + 1) };

        let mut grid = HashMap::new();
        for i in 0..fly_count {
            let row = (i % 2) as usize;
            let mut col = ((i + 1) * spacing) as usize;
            if col >= self.cols {
                col = self.cols - 1;
            }
            let position = Cords(row, col);

            let new_fly = Ship::new_fly();

            grid.insert(position, new_fly);
        }

        grid
    }
}
