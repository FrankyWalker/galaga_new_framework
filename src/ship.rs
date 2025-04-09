use crate::settings::{Settings, Values};
use crate::ship_ai::{AIAction, ShipAI};
use crate::structs::{Cords, RelCords, ShipAction, Timer, ROWS};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub enum Ship {
    Fly(ShipAI, Uuid),
    Explosion(ShipAI, Uuid),
    Bullet(ShipAI, Uuid),
}

impl Ship {
    pub fn display_type(&self) -> &str {
        match self {
            Ship::Fly(_, _) => "fly",
            Ship::Explosion(_, _) => "explosion",
            Ship::Bullet(_, _) => "bullet",
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Ship::Fly(_, id) => *id,
            Ship::Explosion(_, id) => *id,
            Ship::Bullet(_, id) => *id,
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>, settings: &Settings) -> ShipAction {
        match self {
            Ship::Fly(ai, _) => {
                ai.get_action(cords, game_board, settings)
            },
            Ship::Explosion(ai, _) => {
                ai.get_action(cords, game_board, settings)
            },
            Ship::Bullet(ai, _) => {
                ai.get_action(cords, game_board, settings)
            },
        }
    }

    pub fn new_fly() -> Self {
        Self::Fly(
            ShipAI::new(
                vec![
                    AIAction::new_await(AIAction::MoveCautious(RelCords(1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                    AIAction::new_await(AIAction::MoveCautious(RelCords(-1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                ]
            ),
            Uuid::new_v4(),
        )
    }

    pub fn new_bullet(moving_down: bool) -> Self {
        let movement = if moving_down {(1, 0) } else { (-1, 0)};

        Self::Bullet(
            ShipAI::new(
                vec![AIAction::new_await(AIAction::RelativeMove(RelCords(movement.0, movement.1)), |s: &Settings| s.value_stats.laser_speed)]
            ),
            Uuid::new_v4(),
        )
    }

    pub fn new_explosion() -> Self {
        Self::Explosion(
            ShipAI::new(
                vec![AIAction::new_await(AIAction::Remove, |_: &Settings| Duration::from_secs(2))]
            ),
            Uuid::new_v4(),
        )
    }
}

pub struct ShipGrid {
    pub grid: HashMap<Cords, Ship>,
    pub score: u32,
}

impl ShipGrid {
    pub fn new() -> Self {
        ShipGrid {
            grid: HashMap::new(),
            score: 0,
        }
    }


    pub fn move_entity(
        &mut self,
        old_coords: Cords,
        new_coords: Cords,
        wrapped: bool,
    ) -> Result<Option<String>, &'static str> {
        if wrapped {
            if let Some(entity) = self.grid.remove(&old_coords) {
                match entity {
                    Ship::Bullet(_, _) => {
                        return Ok(None);
                    },
                    Ship::Fly(_, _) => {
                        return if old_coords.0 < new_coords.0 && old_coords.0 < ROWS / 2 && new_coords.0 > ROWS / 2 {
                            self.grid.insert(old_coords, entity);
                            Ok(None)
                        } else {
                            self.grid.insert(new_coords, entity);
                            Ok(None)
                        }
                    },
                    _ => {
                        self.grid.insert(old_coords, entity);
                    }
                }
            }
        }

        if new_coords.0 >= ROWS {
            if let Some(entity) = self.grid.remove(&old_coords) {
                if matches!(entity, Ship::Bullet(_, _)) {
                    return Ok(None);
                } else {
                    self.grid.insert(old_coords, entity);
                }
            }
        }

        if let Some(entity) = self.grid.remove(&old_coords) {
            let is_bullet = matches!(entity, Ship::Bullet(_, _));

            if self.grid.contains_key(&new_coords) {
                if is_bullet {
                    let existing_ship = self.grid.remove(&new_coords);
                    let removed_type = existing_ship.as_ref().map(|ship| ship.display_type().to_string());

                    self.grid.insert(new_coords, Ship::new_explosion());

                    Ok(removed_type)
                } else {
                    self.grid.insert(old_coords, entity);
                    Err("target pos in use")
                }
            } else {
                self.grid.insert(new_coords, entity);
                Ok(None)
            }
        } else {
            Err("no entity found at old cords")
        }
    }

    pub fn process_ship_actions(&mut self, settings: &Settings) {
        let entries: Vec<(Cords, Uuid)> = self.grid
            .iter()
            .map(|(&coords, ship)| (coords, ship.get_id()))
            .collect();

        let mut actions_to_preform: Vec<(Cords, ShipAction)> = Vec::new();

        for (coords, _) in &entries {
            if let Some(mut ship) = self.grid.remove(coords) {
                let action = ship.get_action(*coords, &self.grid, settings);

                self.grid.insert(*coords, ship);

                actions_to_preform.push((*coords, action));
            }
        }

        self.execute_actions(&actions_to_preform, settings);
    }


    fn execute_actions(&mut self, actions: &[(Cords, ShipAction)], settings: &Settings) {
        for (coords, action) in actions {
            if !self.grid.contains_key(coords) {
                continue;
            }

            match action {
                ShipAction::Move(new_coords, wrapped) => {
                    let result = self.move_entity(*coords, *new_coords, *wrapped);

                    if let Ok(Some(removed_type)) = result {
                        if removed_type == "fly" {
                            self.score += 100;
                        }
                    }
                },
                ShipAction::Shoot => {
                    let bullet_coords = Cords(coords.0 + 1, coords.1);
                    let bullet = Ship::new_bullet(true);
                    self.grid.insert(bullet_coords, bullet);
                },
                ShipAction::Remove => {
                    if let Some(ship) = self.grid.get(coords) {
                        if ship.display_type() == "fly" {
                            self.score += 100;
                        }
                    }
                    self.grid.remove(coords);
                },
                ShipAction::Nothing => {}
            }
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}
