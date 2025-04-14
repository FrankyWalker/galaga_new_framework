use std::time::Instant;

pub const SIZE: usize = 10;
pub const ROWS: usize = SIZE + 2;
pub const COLUMNS: usize = SIZE + 6;

pub const CELL_SIZE: (u32, u32) = (45, 45);
pub const MARGIN: u32 = 5;
pub const START_X: u32 = 5;
pub const START_Y: u32 = 50;


#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Cords(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub struct RelCords(pub i32, pub i32);


impl RelCords {
    pub fn evaluate(&self, cords: Cords) -> (Cords, bool) {

        let new_cords = (
            (cords.0 as i32 + self.0),
            (cords.1 as i32 + self.1),
        );

        let mut wrapped = false;

        let new_cords = Cords(
            if new_cords.0 < 0 {
                wrapped = true;
                ROWS - 1
            } else if new_cords.0 >= ROWS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.0 as usize
            },
            if new_cords.1 < 0 {
                wrapped = true;
                COLUMNS - 1
            } else if new_cords.1 >= COLUMNS as i32 {
                wrapped = true;
                0
            } else {
                new_cords.1 as usize
            },
        );

        (new_cords, wrapped)
    }
}

#[derive(Clone, Debug)]
pub enum ShipAction {
    Nothing,
    Remove,
    Shoot,
    Move(Cords, bool),
}

#[derive(Clone, Debug)]
pub struct Timer {

    last_action_time: Instant,
    interval_ms: u64,

    duration: u32,
    current: u32,
    active: bool,
    name: String,
}

impl Timer {
    pub fn new(interval_ms: u64) -> Self {
        Timer {
            last_action_time: Instant::now(),
            interval_ms,
            duration: 0,
            current: 0,
            active: true,
            name: String::new(),
        }
    }

    pub fn new_with_duration(interval_ms: u64, duration: u32, name: &str) -> Self {
        Timer {
            last_action_time: Instant::now(),
            interval_ms,
            duration,
            current: 0,
            active: true,
            name: name.to_string(),
        }
    }

    pub fn tick(&mut self) -> bool {
        if !self.active {
            return false;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_action_time).as_millis() as u64;

        if elapsed >= self.interval_ms {
            self.last_action_time = now;

            self.current += 1;

            if self.duration > 0 && self.current >= self.duration {
                self.active = false;
                return true;
            }

            return true;
        }

        false
    }

    pub fn logic_tick(&mut self) -> bool {
        if !self.active {
            return false;
        }

        self.current += 1;

        if self.duration > 0 && self.current >= self.duration {
            self.active = false;
            true
        } else {
            false
        }
    }

    pub fn start(&mut self) {
        self.current = 0;
        self.active = true;
        self.last_action_time = Instant::now();
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.active = true;
        self.last_action_time = Instant::now();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn remaining(&self) -> u32 {
        if !self.active || self.duration == 0 {
            return 0;
        }
        self.duration.saturating_sub(self.current)
    }

    pub fn set_interval(&mut self, interval_ms: u64) {
        self.interval_ms = interval_ms;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}