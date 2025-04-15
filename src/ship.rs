use crate::settings::Settings;
use crate::ship_ai::{AIAction, ShipAI};
use crate::structs::{Cords, RelCords, ShipAction, ROWS};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub trait Ship {
    fn display_type(&self) -> &str;
    fn get_id(&self) -> Uuid;
    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction;
}

pub struct FlyShip {
    ai: ShipAI,
    id: Uuid,
}

impl FlyShip {
    pub fn new() -> Self {
        Self {
            ai: ShipAI::new(
                vec![
                    //   AIAction::new_await(AIAction::MoveCautious(RelCords(1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                    //  AIAction::new_await(AIAction::MoveCautious(RelCords(-1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                ]
            ),
            id: Uuid::new_v4(),
        }
    }
}

impl Ship for FlyShip {
    fn display_type(&self) -> &str {
        "fly"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub struct TikiFlyShip {
    ai: ShipAI,
    id: Uuid,
}

impl TikiFlyShip {
    pub fn new() -> Self {
        Self {
            ai: ShipAI::new(
                vec![
                    //   AIAction::new_await(AIAction::MoveCautious(RelCords(1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                    //  AIAction::new_await(AIAction::MoveCautious(RelCords(-1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                ]
            ),
            id: Uuid::new_v4(),
        }
    }
}

impl Ship for TikiFlyShip {
    fn display_type(&self) -> &str {
        "tiki_fly"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub struct NorthropFlyShip {
    ai: ShipAI,
    id: Uuid,
}

impl NorthropFlyShip {
    pub fn new() -> Self {
        Self {
            ai: ShipAI::new(
                vec![
                 //   AIAction::new_await(AIAction::MoveCautious(RelCords(1, 1)), |s: &Settings| s.value_stats.fly_speed),
                 AIAction::RandomShoot,
                  //  AIAction::new_await(AIAction::MoveCautious(RelCords(-1, 1)), |s: &Settings| s.value_stats.fly_speed),
                 AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                 AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                 AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                 AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                 AIAction::RandomShoot,
                ]
            ),
            id: Uuid::new_v4(),
        }
    }
}

impl Ship for NorthropFlyShip {
    fn display_type(&self) -> &str {
        "northrop_fly"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub struct B2FlyShip {
    ai: ShipAI,
    id: Uuid,
}

impl B2FlyShip {
    pub fn new() -> Self {
        Self {
            ai: ShipAI::new(
                vec![
                    //   AIAction::new_await(AIAction::MoveCautious(RelCords(1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                    //  AIAction::new_await(AIAction::MoveCautious(RelCords(-1, 1)), |s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::new_await(AIAction::Nothing,|s: &Settings| s.value_stats.fly_speed),
                    AIAction::RandomShoot,
                ]
            ),
            id: Uuid::new_v4(),
        }
    }
}

impl Ship for B2FlyShip {
    fn display_type(&self) -> &str {
        "b2_fly"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub struct ExplosionShip {
    ai: ShipAI,
    id: Uuid,
}

impl ExplosionShip {
    pub fn new() -> Self {
        Self {
            ai: ShipAI::new(
                vec![AIAction::new_await(AIAction::Remove, |_: &Settings| Duration::from_secs(1))]
            ),
            id: Uuid::new_v4(),
        }
    }
}

impl Ship for ExplosionShip {
    fn display_type(&self) -> &str {
        "explosion"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub struct BulletShip {
    ai: ShipAI,
    id: Uuid,
    moving_down: bool,
}

impl BulletShip {
    pub fn new(moving_down: bool) -> Self {
        let movement = if moving_down { (1, 0) } else { (-1, 0) };

        Self {
            ai: ShipAI::new(
                vec![AIAction::new_await(AIAction::RelativeMove(RelCords(movement.0, movement.1)), |s: &Settings| s.value_stats.laser_speed)]
            ),
            id: Uuid::new_v4(),
            moving_down,
        }
    }
}

impl Ship for BulletShip {
    fn display_type(&self) -> &str {
        "bullet"
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>, settings: &Settings) -> ShipAction {
        self.ai.get_action(cords, game_board, settings)
    }
}

pub fn new_fly_ship() -> Box<dyn Ship> {
    Box::new(FlyShip::new())
}

pub fn new_tiki_fly_ship() -> Box<dyn Ship> {
    Box::new(TikiFlyShip::new())
}

pub fn new_northrop_fly_ship() -> Box<dyn Ship> {
    Box::new(NorthropFlyShip::new())
}

pub fn new_b2_fly_ship() -> Box<dyn Ship> {
    Box::new(B2FlyShip::new())
}

pub fn new_explosion_ship() -> Box<dyn Ship> {
    Box::new(ExplosionShip::new())
}

pub fn new_bullet_ship(moving_down: bool) -> Box<dyn Ship> {
    Box::new(BulletShip::new(moving_down))
}

pub struct ShipGrid {
    pub grid: HashMap<Cords, Box<dyn Ship>>,
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
            if let Some(mut entity) = self.grid.remove(&old_coords) {
                if entity.display_type() == "bullet" {
                    return Ok(None);
                } else if entity.display_type() == "fly"
                    || entity.display_type() == "tiki_fly"
                    || entity.display_type() == "northrop_fly"
                    || entity.display_type() == "b2_fly" {
                    return if old_coords.0 < new_coords.0 && old_coords.0 <ROWS / 2 && new_coords.0 > ROWS / 2 {
                        self.grid.insert(old_coords, entity);
                        Ok(None)
                    } else {
                        self.grid.insert(new_coords, entity);
                        Ok(None)
                    }
                } else {
                    self.grid.insert(old_coords, entity);
                }
            }
        }

        if new_coords.0 >= ROWS {
            if let Some(entity) = self.grid.remove(&old_coords) {
                if entity.display_type() == "bullet" {
                    return Ok(None);
                } else {
                    self.grid.insert(old_coords, entity);
                }
            }
        }

        if let Some(entity) = self.grid.remove(&old_coords) {
            let is_bullet = entity.display_type() == "bullet";

            if self.grid.contains_key(&new_coords) {
                if is_bullet {
                    let existing_ship = self.grid.remove(&new_coords);
                    let removed_type = existing_ship.as_ref().map(|ship| ship.display_type().to_string());

                    self.grid.insert(new_coords, new_explosion_ship());

                    if let Some(removed_type_str) = &removed_type {
                        match removed_type_str.as_str() {
                            "fly" => self.score += 100,
                            "tiki_fly" => self.score += 150,
                            "northrop_fly" => self.score += 200,
                            "b2_fly" => self.score += 300,
                            _ => {}
                        }
                    }

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

        self.execute_actions(&actions_to_preform);
    }

    fn execute_actions(&mut self, actions: &[(Cords, ShipAction)]) {
        for (coords, action) in actions {
            if !self.grid.contains_key(coords) {
                continue;
            }

            match action {
                ShipAction::Move(new_coords, wrapped) => {
                    let result = self.move_entity(*coords, *new_coords, *wrapped);

                    if let Ok(Some(removed_type)) = result {
                    }
                },
                ShipAction::Shoot => {
                    let bullet_coords = Cords(coords.0 + 1, coords.1);
                    self.grid.insert(bullet_coords, new_bullet_ship(true));
                },
                ShipAction::Remove => {
                    if let Some(ship) = self.grid.get(coords) {
                        match ship.display_type() {
                            "fly" => self.score += 100,
                            "tiki_fly" => self.score += 150,
                            "northrop_fly" => self.score += 200,
                            "b2_fly" => self.score += 300,
                            _ => {}
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
