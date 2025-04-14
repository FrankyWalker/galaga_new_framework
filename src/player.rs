use crate::ship::{Ship, new_bullet_ship};
use crate::structs::{Cords, Timer, COLUMNS, ROWS};
use crate::settings::Settings;
use std::collections::HashMap;
use std::time::{Instant, Duration};

#[derive(Clone)]
pub struct Player {
    pub lives: u8,
    pub current_position: Option<Cords>,
    pub start_position: Cords,
    pub movement_direction: i8,
    pub settings: Settings,
    pub is_dead: bool,
    pub blink_timer: Timer,
    pub blink_count: u8,
    pub last_shot_time: Option<Instant>,  // Track when the last shot was fired
}

impl Player {
    pub fn new() -> Self {
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            lives: 5,
            current_position: Some(start_position),
            start_position,
            movement_direction: 1,
            settings: Settings::new(),
            is_dead: false,
            blink_timer: Timer::new_with_duration(50, 15, "blink_timer"),
            blink_count: 0,
            last_shot_time: None,
        }
    }

    pub fn move_left(&mut self) {
        if self.is_dead {
            return;
        }

        if let Some(current_pos) = &mut self.current_position {
            if current_pos.1 > 0 {
                current_pos.1 -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.is_dead {
            return;
        }

        if let Some(current_pos) = &mut self.current_position {
            if current_pos.1 < COLUMNS - 1 {
                current_pos.1 += 1;
            }
        }
    }

    pub fn shoot(&mut self, grid: &mut HashMap<Cords, Box<dyn Ship>>) -> bool {
        if self.is_dead {
            return false;
        }

        // Check if enough time has passed since the last shot (200ms cooldown)
        let now = Instant::now();
        if let Some(last_time) = self.last_shot_time {
            if now.duration_since(last_time) < Duration::from_millis(2000) {
                return false; // Not enough time has passed
            }
        }

        match self.current_position {
            Some(pos) => {
                let bullet_coords = Cords(pos.0.saturating_sub(1), pos.1);
                if grid.contains_key(&bullet_coords) {
                    false
                } else {
                    let bullet = new_bullet_ship(false);
                    grid.insert(bullet_coords, bullet);
                    // Update the last shot time
                    self.last_shot_time = Some(now);
                    true
                }
            }
            None => false,
        }
    }

    pub fn handle_collision(&mut self, grid: &mut HashMap<Cords, Box<dyn Ship>>, pos: Cords, score: &mut u32, invincible: bool) -> bool {
        if self.is_dead || !grid.contains_key(&pos) || invincible {
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

        self.is_dead = true;
        self.current_position = None;
        self.blink_count = 0;
        self.blink_timer.reset();

        if self.lives == 0 {
            self.reset();
            Some(5)
        } else {
            Some(self.lives)
        }
    }

    pub fn reset(&mut self) {
        self.lives = 5;
        self.current_position = None;
        self.is_dead = true;
        self.blink_count = 0;
        self.blink_timer.reset();
        self.movement_direction = 1;
        self.last_shot_time = None;  // Reset the last shot time when player resets
    }

    pub fn update(&mut self) {
        if self.is_dead {
            if self.blink_timer.tick() {
                if self.current_position.is_none() {
                    self.current_position = Some(self.start_position);
                } else {
                    self.current_position = None;
                    self.blink_count += 1;
                }

                self.blink_timer.reset();

                if self.blink_count >= 3 {
                    self.is_dead = false;
                    self.current_position = Some(self.start_position);
                }
            }
        }
    }
}