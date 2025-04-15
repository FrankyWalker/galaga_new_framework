// ship_ai.rs
use crate::settings::Settings;
use crate::ship::Ship;
use crate::structs::{Cords, RelCords, ShipAction, ROWS};
use rand::Rng;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct AIActionResult {
    pub move_on_to_next_action: bool,
    pub ship_action: ShipAction,
}


pub struct ShipAI {
    pub actions: Vec<AIAction>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(actions: Vec<AIAction>) -> Self {
        ShipAI {
            actions,
            action_index: 0,
        }
    }

    pub fn get_action(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Box<dyn Ship>>,
        settings: &Settings,
    ) -> ShipAction {
        if self.actions.is_empty() {
            return ShipAction::Nothing;
        }

        let result = self.actions[self.action_index].evaluate(cords, game_board, settings);
        if result.move_on_to_next_action {
            if self.action_index == self.actions.len() - 1 {
                self.action_index = 0;
            } else {
                self.action_index += 1;
            }
        }
        result.ship_action
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    ShootPositionAvailable(RelCords),
}

impl Condition {
    pub fn evaluate(&self, cords: Cords, game_board: &HashMap<Cords, Box<dyn Ship>>) -> bool {
        match self {
            Condition::ShipExists(ref target_cords) => {
                game_board.contains_key(target_cords)
            }
            Condition::PositionAvailable(rel_cords) => {
                let (target_cords, _) = rel_cords.evaluate(cords);
                game_board.get(&target_cords).is_none()
            }
            Condition::ShootPositionAvailable(rel_cords) => {
                let (target_cords, _) = rel_cords.evaluate(cords);
                game_board.get(&target_cords).is_none()
            }
        }
    }
}

pub enum AIAction {
    Nothing,
    Remove,
    Shoot,
    RandomShoot,
    Move(Cords),
    MoveCautious(RelCords),
    ShootOrNothing,
    RelativeMove(RelCords),
    AwaitAction(Box<AIAction>, Option<Instant>, Box<dyn Fn(&Settings) -> Duration>),
}

impl AIAction {
    pub fn new_await(action: AIAction, get_duration: impl Fn(&Settings) -> Duration  + 'static) -> Self {
        AIAction::AwaitAction(Box::new(action), None, Box::new(get_duration))
    }

    pub fn evaluate(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Box<dyn Ship>>,
        settings: &Settings,
    ) -> AIActionResult {
        match self {
            AIAction::Remove => AIActionResult {
                move_on_to_next_action: true,
                ship_action: ShipAction::Remove,
            },

            AIAction::Shoot => AIActionResult {
                move_on_to_next_action: true,
                ship_action: ShipAction::Shoot,
            },

            AIAction::RandomShoot => {
                let randomness = settings.value_stats.shooting_randomness;
                if randomness > 0 {
                    let mut rng = rand::thread_rng();

                    let threshold = match randomness {
                        1 => 90,
                        2 => 80,
                        3 => 70,
                        4 => 60,
                        5 => 50,
                        6 => 40,
                        7 => 30,
                        8 => 20,
                        9 => 10,
                        10 => 5,
                        _ => 50,
                    };


                    let mut can_shoot = true;
                    for row in (cords.0 + 1)..ROWS {
                        let check_coords = Cords(row, cords.1);
                        if let Some(ship) = game_board.get(&check_coords) {
                            let ship_type = ship.display_type();
                            if ship_type == "fly" || ship_type == "tiki_fly" ||
                                ship_type == "northrop_fly" || ship_type == "b2_fly" {
                                can_shoot = false;
                                break;
                            }
                        }
                    }

                    let condition = Condition::ShootPositionAvailable(RelCords(1, 0));
                    if condition.evaluate(cords, game_board) && can_shoot && rng.gen_range(1..=100) <= threshold {
                        AIActionResult {
                            move_on_to_next_action: true,
                            ship_action: ShipAction::Shoot,
                        }
                    } else {
                        AIActionResult {
                            move_on_to_next_action: true,
                            ship_action: ShipAction::Nothing,
                        }
                    }
                } else {
                    AIActionResult {
                        move_on_to_next_action: true,
                        ship_action: ShipAction::Nothing,
                    }
                }
            },

            AIAction::Move(cords) => AIActionResult {
                move_on_to_next_action: true,
                ship_action: ShipAction::Move(*cords, false),
            },

            AIAction::MoveCautious(rel_cords) => {
                if settings.value_stats.fly_move {
                    let condition = Condition::PositionAvailable(rel_cords.clone());
                    if condition.evaluate(cords, game_board) {
                        let (new_cords, wrap) = rel_cords.evaluate(cords);

                        let mut safe_to_move = true;

                        for (&check_coords, ship) in game_board.iter() {
                            if check_coords == new_cords {
                                let ship_type = ship.display_type();
                                if ship_type == "fly" {
                                    safe_to_move = false;
                                    break;
                                } else if ship_type == "bullet" {
                                    safe_to_move = false;
                                    break;
                                }
                            }
                        }
                        if safe_to_move {
                            AIActionResult {
                                move_on_to_next_action: true,
                                ship_action: ShipAction::Move(new_cords, wrap),
                            }
                        } else {
                            AIActionResult {
                                move_on_to_next_action: true,
                                ship_action: ShipAction::Nothing,
                            }
                        }
                    } else {
                        AIActionResult {
                            move_on_to_next_action: true,
                            ship_action: ShipAction::Nothing,
                        }
                    }
                } else {
                    AIActionResult {
                        move_on_to_next_action: true,
                        ship_action: ShipAction::Nothing,
                    }
                }
            },
            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrapped) = rel_cords.evaluate(cords);
                if wrapped {
                    println!("Wrapped: {}", wrapped);
                }
                AIActionResult {
                    move_on_to_next_action: true,
                    ship_action: ShipAction::Move(new_cords, wrapped),
                }
            },
            AIAction::ShootOrNothing => {
                let mut can_shoot = true;

                for row in (cords.0 + 1)..ROWS {
                    let check_coords = Cords(row, cords.1);

                    if let Some(ship) = game_board.get(&check_coords) {
                        if ship.display_type() == "fly" {
                            can_shoot = false;
                            break;
                        }
                    }
                }

                if can_shoot {
                    AIActionResult {
                        move_on_to_next_action: true,
                        ship_action: ShipAction::Shoot,
                    }
                } else {
                    AIActionResult {
                        move_on_to_next_action: true,
                        ship_action: ShipAction::Nothing,
                    }
                }
            },
            AIAction::AwaitAction(ai_action, start_time, get_duration) => {
                if start_time.is_none() {
                    *start_time = Some(Instant::now());
                }

                let elapsed = start_time.unwrap().elapsed();

                if elapsed >= get_duration(&settings) {
                    let result = ai_action.evaluate(cords, game_board, settings);
                    if result.move_on_to_next_action {
                        *start_time = None;
                    }
                    result
                } else {
                    AIActionResult {
                        move_on_to_next_action: false,
                        ship_action: ShipAction::Nothing,
                    }
                }
            },
            AIAction::Nothing => AIActionResult {
                move_on_to_next_action: true,
                ship_action: ShipAction::Nothing,
            },
        }
    }
}

