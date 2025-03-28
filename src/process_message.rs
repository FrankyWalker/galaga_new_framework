use rust_on_rails::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use crate::structs::Timer;
use crate::ship::Ship;
use crate::structs::{Cords, COLUMNS};
use crate::player::Player;
use crate::settings::GameSettings;
use crate::server::PressurePadData;
use crate::server::PadType;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Receiver;

pub async fn process_message(
    rx_arc: Arc<Mutex<Receiver<PressurePadData>>>,
    player: &mut Player,
    settings: &mut GameSettings,
    grid: &mut HashMap<Cords, Ship>
) -> Option<()> {
    let message = {
        let mut rx = rx_arc.lock().await;
        tokio::time::timeout(std::time::Duration::from_millis(10), rx.recv()).await
    };

    match message {
        Ok(Some(message)) => {
            match message.pad_type {
                PadType::Left => {
                    if let Some(current_pos) = &mut player.current_position {
                        if current_pos.1 > 0 {
                            current_pos.1 -= 1;
                        }
                    }
                },
                PadType::Right => {
                    if let Some(current_pos) = &mut player.current_position {
                        if current_pos.1 < COLUMNS - 1 {
                            current_pos.1 += 1;
                        }
                    }
                },
                PadType::Shoot => {
                    if settings.can_shoot(true) {
                        if let Some(pos) = player.current_position {
                            let bullet_coords = Cords(pos.0.saturating_sub(1), pos.1);

                            if !grid.contains_key(&bullet_coords) {
                                let mut bullet = Ship::new_bullet(false, 1);

                                if let Ship::Bullet(ref mut ai, _, _) = bullet {
                                    let speed_factor = 11 - settings.laser_speed;
                                    ai.timer = Timer::new(speed_factor as u64);
                                }

                                grid.insert(bullet_coords, bullet);
                            }
                        }
                    }
                },
            }
            Some(())
        },
        Ok(None) => None,
        Err(_) => None
    }
}