use crate::ship::Ship;
use crate::structs::{Cords, Timer, COLUMNS, ROWS};
use crate::settings::Settings;
use std::collections::HashMap;


#[derive(Clone)]
pub struct Player {
    pub lives: u8,
    pub current_position: Option<Cords>,
    pub start_position: Cords,
    pub death_timer: Timer,
    pub movement_direction: i8,
    pub settings: Settings,
}

impl Player {
    pub fn new() -> Self {
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            lives: 5,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(30),
            movement_direction: 1,
            settings: Settings::new(),
        }
    }

    pub fn move_left(&mut self) {
        if let Some(current_pos) = &mut self.current_position {
            if current_pos.1 > 0 {
                current_pos.1 -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if let Some(current_pos) = &mut self.current_position {
            if current_pos.1 < COLUMNS - 1 {
                current_pos.1 += 1;
            }
        }
    }

    pub fn shoot(&mut self, grid: &mut HashMap<Cords, Ship>) -> bool {
        match self.current_position {
            Some(pos) => {
                let bullet_coords = Cords(pos.0.saturating_sub(1), pos.1);
                if grid.contains_key(&bullet_coords) {
                    false
                } else {
                    let bullet = Ship::new_bullet(false);
                    grid.insert(bullet_coords, bullet);
                    true
                }
            }
            None => false,
        }
    }

    pub fn handle_collision(&mut self, grid: &mut HashMap<Cords, Ship>, pos: Cords, score: &mut u32, invincible: bool) -> bool {
        if !grid.contains_key(&pos) || invincible {
            return false;
        }

        let is_fly = match grid.get(&pos) {
            Some(ship) => ship.display_type() == "fly",
            None => false
        };

        let remaining_lives = self.decrease_lives();

        match remaining_lives {
            Some(5) => true,
            Some(_) => {
                if is_fly {
                    *score += 100;
                }
                grid.remove(&pos);
                false
            },
            None => false
        }
    }

    pub fn decrease_lives(&mut self) -> Option<u8> {
        self.lives -= 1;

        if self.lives == 0 {
            self.current_position = None;
            self.reset();
            Some(5)
        } else {
            self.current_position = None;
            self.current_position = Some(self.start_position);
            Some(self.lives)
        }
    }

    pub fn reset(&mut self) {
        self.lives = 5;
        self.current_position = Some(self.start_position);
        self.death_timer = Timer::new(1);
        self.movement_direction = 1;
    }
}
