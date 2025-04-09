pub const SIZE: usize = 10;
pub const ROWS: usize = SIZE + 2;
pub const COLUMNS: usize = SIZE + 4;

pub const CELL_SIZE: (u32, u32) = (50, 50);
pub const MARGIN: u32 = 5;
pub const START_X: u32 = 5;
pub const START_Y: u32 = 50;


#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Cords(pub usize, pub usize);

#[derive(Clone, Debug)]
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

#[derive(Clone)]
pub struct Timer {
    last_action_time: std::time::Instant,
    interval_ms: u64,
}

impl Timer {
    pub fn new(interval_ms: u64) -> Self {
        Timer {
            last_action_time: std::time::Instant::now(),
            interval_ms,
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_action_time).as_millis() as u64;

        if elapsed >= self.interval_ms {
            self.last_action_time = now;
            true
        } else {
            false
        }
    }
}
