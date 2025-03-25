use crate::structs::{ROWS, COLUMNS, Cords, Timer, ShipAction};
use crate::ship::Ship;
use std::collections::HashMap;
use rust_on_rails::canvas::{Area, CanvasItem, Shape};

#[derive(Clone)]
pub struct Player {
    pub lives: u8,
    pub current_position: Option<Cords>,
    pub start_position: Cords,
    pub death_timer: Timer,
    pub display_color: &'static str,
    pub movement_direction: i8,
    pub movement_timer: Timer,
    pub shoot_timer: Timer,
}

impl Player {
    pub fn new() -> Self {
        let start_position = Cords(ROWS - 2, COLUMNS / 2);
        Player {
            display_color: "BF40BF",
            lives: 5,
            current_position: Some(start_position),
            start_position,
            death_timer: Timer::new(30),
            movement_direction: 1,
            movement_timer: Timer::new(15),
            shoot_timer: Timer::new(4),
        }
    }



    pub fn move_up(&mut self) {
        if let Some(pos) = self.current_position {
            let new_pos = Cords(pos.0.saturating_sub(1), pos.1);
            self.move_to(new_pos);
        }
    }

    pub fn move_left(&mut self) {
        if let Some(pos) = self.current_position {
            let new_pos = Cords(pos.0, pos.1.saturating_sub(1));
            self.move_to(new_pos);
        }
    }

    pub fn move_down(&mut self) {
        if let Some(pos) = self.current_position {
            let new_pos = Cords(pos.0 + 1, pos.1);
            self.move_to(new_pos);
        }
    }

    pub fn move_right(&mut self) {
        if let Some(pos) = self.current_position {
            let new_pos = Cords(pos.0, pos.1 + 1);
            self.move_to(new_pos);
        }
    }

    pub fn move_to(&mut self, new_position: Cords) {
        self.current_position = Some(new_position);
    }

    pub fn handle_collision(&mut self) -> Option<u8> {
        self.lives -= 1;

        if self.lives == 0 {
            self.current_position = None;
            self.reset();
            Some(5)
        } else {
            self.current_position = None;
            Some(self.lives)
        }
    }

    pub fn reset(&mut self) {
        self.lives = 5;
        self.current_position = Some(self.start_position);
        self.death_timer = Timer::new(30);
        self.movement_direction = 1;
    }

    pub fn respawn(&mut self, can_respawn: bool) {
        if self.current_position.is_none() && self.death_timer.tick() {
            if can_respawn {
                self.move_to(self.start_position);
            }
        }
    }

    pub fn update(&mut self, grid: &mut HashMap<Cords, Ship>) -> Option<ShipAction> {

        if self.current_position.is_none() {
            return None;
        }

        let current_pos = self.current_position.unwrap();

        if self.movement_timer.tick() {
            let new_column = (current_pos.1 as i32 + self.movement_direction as i32) as usize;

            if new_column >= COLUMNS - 1 {
                self.movement_direction = -1;
            } else if new_column <= 0 {
                self.movement_direction = 1;
            }

            let next_column = (current_pos.1 as i32 + self.movement_direction as i32) as usize;
            let new_pos = Cords(current_pos.0, next_column);

            if !grid.contains_key(&new_pos) {
                self.current_position = Some(new_pos);
                return Some(ShipAction::Move(new_pos, false));
            }
        }

        if self.shoot_timer.tick() {
            let bullet_pos = Cords(current_pos.0 - 1, current_pos.1);
                return Some(ShipAction::Shoot);

        }

        None
    }
}



pub fn integrate_player_to_game(
    player: &mut Player,
    grid: &mut HashMap<Cords, Ship>,
    items: &mut Vec<CanvasItem>,
    cell_size: (u32, u32),
    margin: u32
) {



    if let Some(pos) = player.current_position {
        let static_color = Box::leak(player.display_color.to_string().into_boxed_str());
        let x = pos.1 as u32 * (cell_size.0 + margin);
        let y = pos.0 as u32 * (cell_size.1 + margin);

        items.push(CanvasItem::Shape(
            Area((x, y), None),
            Shape::RoundedRectangle(0, cell_size, 10),
            static_color,
            255,
        ));
    }

    let can_respawn = match player.current_position {
        Some(_) => true,
        None => {

            !grid.contains_key(&player.start_position)
        }
    };

    player.respawn(can_respawn);
}
