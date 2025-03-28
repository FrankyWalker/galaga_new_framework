use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use crate::ship_actions::add_ship;

pub struct GameSettings {
    pub invincible: bool,
    pub fly_move: bool,
    pub laser_shoot: bool,

    pub number_of_flies: u32,
    pub fly_speed: u32,
    pub laser_speed: u32,

    last_move_time: Instant,
    last_shot_time: Instant,
    last_fly_move_time: Instant,
    pub move_cooldown: Duration,
    pub shot_cooldown: Duration,
    pub fly_move_cooldown: Duration,
}

impl GameSettings {
    pub fn new() -> Self {
        let now = Instant::now();
        let default = Self {
            invincible: false,
            fly_move: true,
            laser_shoot: true,
            number_of_flies: 10,
            fly_speed: 1,
            laser_speed: 1,
            last_move_time: now,
            last_shot_time: now,
            last_fly_move_time: now,
            move_cooldown: Duration::from_millis(250),
            shot_cooldown: Duration::from_millis(500),
            fly_move_cooldown: Duration::from_millis(500),
        };

        let mut settings = default;
        settings.update_cooldowns();
        settings
    }

    // Existing methods remain the same as in the previous implementation
    pub fn calculate_fly_value(&self) -> u32 {
        self.number_of_flies * 2
    }

    pub fn set_fly_speed(&mut self, value: u32) -> u32 {
        self.fly_speed = if value < 1 {
            1
        } else if value > 10 {
            10
        } else {
            value
        };

        self.update_cooldowns();
        self.fly_speed
    }

    pub fn get_fly_speed(&self) -> u32 {
        self.fly_speed
    }

    pub fn set_laser_speed(&mut self, value: u32) -> u32 {
        self.laser_speed = if value < 1 {
            1
        } else if value > 6 {
            6
        } else {
            value
        };

        self.update_cooldowns();
        self.laser_speed
    }

    pub fn get_laser_speed(&self) -> u32 {
        self.laser_speed
    }

    pub fn update_cooldowns(&mut self) {
        self.move_cooldown = Duration::from_millis(250);
        self.shot_cooldown = Duration::from_millis(500);
        self.fly_move_cooldown = Duration::from_millis(500);
    }

    pub fn set_invincible(&mut self, enabled: bool) {
        self.invincible = enabled;
    }

    pub fn can_move(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_move_time) >= self.move_cooldown {
            self.last_move_time = now;
            true
        } else {
            false
        }
    }

    pub fn can_flies_move(&mut self) -> bool {
        if !self.fly_move {
            return false;
        }

        let now = Instant::now();
        if now.duration_since(self.last_fly_move_time) >= self.fly_move_cooldown {
            self.last_fly_move_time = now;
            true
        } else {
            false
        }
    }

    pub fn can_shoot(&mut self, is_upward: bool) -> bool {
        if !self.laser_shoot {
            return is_upward;
        }

        let now = Instant::now();
        if now.duration_since(self.last_shot_time) >= self.shot_cooldown {
            self.last_shot_time = now;
            true
        } else {
            false
        }
    }

    pub fn set_number_of_flies(&mut self, count: u32) {
        self.number_of_flies = count.max(1).min(10);
    }

    pub fn set_fly_movement(&mut self, enabled: bool) {
        self.fly_move = enabled;
    }

    pub fn set_laser_shooting(&mut self, enabled: bool) {
        self.laser_shoot = enabled;
    }
}

pub fn spawn_initial_flies(
    settings: &GameSettings,
    grid: &mut std::collections::HashMap<crate::structs::Cords, crate::ship::Ship>,
    rows: u32,
    cols: u32
) {
    // Remove existing flies from the grid
    grid.retain(|_, ship| {
        !matches!(ship, crate::ship::Ship::Fly(_, _, _, _))
    });

    let fly_count = settings.number_of_flies;

    let spacing = cols / (fly_count + 1);

    let spacing = if spacing == 0 { 1 } else { spacing };

    for i in 0..fly_count {
        let row = i % 2;

        let col = (i + 1) * spacing;

        let col = if col >= cols { cols - 1 } else { col };

        let new_fly = crate::ship::Ship::new_fly_with_randomness(
            9,
            settings.get_fly_speed() as u64
        );

        grid.insert(
            crate::structs::Cords(row as usize, col as usize),
            new_fly
        );
    }
}

pub fn ensure_flies_on_grid(
    settings: &GameSettings,
    grid: &mut std::collections::HashMap<crate::structs::Cords, crate::ship::Ship>,
    rows: u32,
    cols: u32
) {
    // Count the number of fly ships currently on the grid
    let current_fly_count = grid.values()
        .filter(|ship| matches!(ship, crate::ship::Ship::Fly(_, _, _, _)))
        .count();

    // If the number of flies is less than the configured number, spawn new flies
    if current_fly_count < settings.number_of_flies as usize {
        spawn_initial_flies(settings, grid, rows, cols);
    }
}

