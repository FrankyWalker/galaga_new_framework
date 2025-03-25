use uuid::Uuid;
use std::collections::HashMap;
use crate::structs::{Cords, Timer, ShipAction, RelCords, ROWS};
use rand::Rng;

pub enum Ship {
    Fly(ShipAI, bool, Uuid, u32),
    Explosion(ShipAI, bool, Uuid),
    Bullet(ShipAI, bool, Uuid),
}

impl Ship {
    pub fn display_type(&self) -> &str {
        match self {
            Ship::Fly(_, _, _, _) => "fly",
            Ship::Explosion(_, _, _) => "explosion",
            Ship::Bullet(_, _, _) => "bullet",
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Ship::Fly(_, _, id, _) => *id,
            Ship::Explosion(_, _, id) => *id,
            Ship::Bullet(_, _, id) => *id,
        }
    }

    pub fn get_action(&mut self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> ShipAction {
        let action = match self {
            Ship::Fly(ai, _, _, shooting_randomness) => {
                let action = ai.get_action(cords, game_board, Some(*shooting_randomness));
                action
            },
            Ship::Explosion(ai, _, _) => {
                let action = ai.get_action(cords, game_board, None);
                action
            },
            Ship::Bullet(ai, _, _) => {
                let raw_action = ai.get_action(cords, game_board, None);

                let action = match raw_action {
                    ShipAction::Move(new_cords, wrapped) => {
                        if (cords.0 == 0 && new_cords.0 == 0) ||
                            (cords.0 == ROWS - 1 && new_cords.0 == ROWS - 1) {
                            ShipAction::Remove
                        } else {
                            ShipAction::Move(new_cords, wrapped)
                        }
                    },
                    other => other,
                };

                action
            },
        };
        action
    }

    pub fn wrap(&self) -> bool {
        match self {
            Ship::Fly(_, wrap, _, _) => *wrap,
            Ship::Explosion(_, wrap, _) => *wrap,
            Ship::Bullet(_, _, _) => false, 
        }
    }

    pub fn new_fly_with_randomness(shooting_randomness: u32, fly_speed: u64) -> Self {
        let randomness = shooting_randomness.clamp(1, 10);

        Self::Fly(
            ShipAI::new(
                fly_speed,
                vec![
                    AIAction::MoveOrNothing(RelCords(1, -1)),
                    AIAction::RandomShoot,
                    AIAction::MoveOrNothing(RelCords(-1, -1)),
                    AIAction::RandomShoot,
                ]
            ),
            true,
            Uuid::new_v4(),
            randomness,
        )
    }

    pub fn new_fly() -> Self {

        Self::new_fly_with_randomness(9, 1)  // Default to speed 1
    }

    pub fn new_bullet(moving_down: bool, bullet_speed: u64) -> Self {
        let movement = if moving_down { RelCords(1, 0) } else { RelCords(-1, 0) };
        Self::Bullet(
            ShipAI::new(
                bullet_speed,  // Use the speed passed in
                vec![AIAction::RelativeMove(movement)]
            ),
            false,
            Uuid::new_v4(),
        )
    }

    pub fn new_explosion() -> Self {
        Self::Explosion(
            ShipAI::new(
                1,
                vec![AIAction::Remove]
            ),
            false,
            Uuid::new_v4(),
        )
    }
}

pub struct ShipAI {
    pub timer: Timer,
    pub actions: Vec<AIAction>,
    pub action_index: usize,
}

impl ShipAI {
    pub fn new(action_interval: u64, actions: Vec<AIAction>) -> Self {
        ShipAI {
            timer: Timer::new(action_interval),
            actions,
            action_index: 0,
        }
    }

    pub fn get_ai_action(&mut self) -> AIAction {
        if self.actions.is_empty() {
            return AIAction::Nothing;
        }

        if self.timer.tick() {
            let action = self.actions[self.action_index].clone();
            if self.action_index == self.actions.len() - 1 {
                self.action_index = 0;
            } else {
                self.action_index += 1;
            }
            action
        } else {
            AIAction::Nothing
        }
    }

    pub fn get_action(
        &mut self,
        cords: Cords,
        game_board: &HashMap<Cords, Ship>,
        shooting_randomness: Option<u32>,
    ) -> ShipAction {
        self.get_ai_action().to_ship_action(cords, game_board, shooting_randomness)
    }
}

pub enum Condition {
    ShipExists(Cords),
    PositionAvailable(RelCords),
    ShootPositionAvailable(RelCords),
}

impl Condition {
    pub fn evaluate(&self, cords: Cords, game_board: &HashMap<Cords, Ship>) -> bool {
        match self {
            Condition::ShipExists(ref target_cords) => {
                game_board.contains_key(target_cords)
            }
            Condition::PositionAvailable(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none()
            }
            Condition::ShootPositionAvailable(rel_cords) => {
                game_board.get(&rel_cords.evaluate(cords).0).is_none()
            }
        }
    }
}

#[derive(Clone)]
pub enum AIAction {
    Nothing,
    Remove,
    Shoot,
    RandomShoot,
    Move(Cords),
    MoveOrNothing(RelCords),
    ShootOrNothing,
    RelativeMove(RelCords),
}

impl AIAction {
    pub fn to_ship_action(
        self,
        cords: Cords,
        game_board: &HashMap<Cords, Ship>,
        shooting_randomness: Option<u32>,
    ) -> ShipAction {
        match self {
            AIAction::Remove => {
                ShipAction::Remove
            }

            AIAction::Shoot => {
                ShipAction::Shoot
            }

            AIAction::RandomShoot => {
                if let Some(randomness) = shooting_randomness {
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

                    let condition = Condition::ShootPositionAvailable(RelCords(1, 0));
                    if condition.evaluate(cords, game_board) && rng.gen_range(1..=100) <= threshold {
                        ShipAction::Shoot
                    } else {
                        ShipAction::Nothing
                    }
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::Move(cords) => {
                ShipAction::Move(cords, false)
            }

            AIAction::MoveOrNothing(rel_cords) => {
                let condition = Condition::PositionAvailable(rel_cords.clone());
                if condition.evaluate(cords, game_board) {
                    let (new_cords, wrap) = rel_cords.evaluate(cords);
                    ShipAction::Move(new_cords, wrap)
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::RelativeMove(rel_cords) => {
                let (new_cords, wrapped) = rel_cords.evaluate(cords);
                ShipAction::Move(new_cords, wrapped)
            }

            AIAction::ShootOrNothing => {
                let condition = Condition::ShootPositionAvailable(RelCords(1, 0));
                if condition.evaluate(cords, game_board) {
                    ShipAction::Shoot
                } else {
                    ShipAction::Nothing
                }
            }

            AIAction::Nothing => {
                ShipAction::Nothing
            }
        }
    }
}
