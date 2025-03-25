use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::settings::GameSettings;
use crate::ship::Ship;
use crate::structs::{Cords, ShipAction};
use crate::structs::Timer;

pub fn add_ship(
    ship_type: &str,
    row: u32,
    col: u32,
    grid: &mut HashMap<Cords, Ship>,
) {
    let position = Cords(row as usize, col as usize);
    if grid.contains_key(&position) {
        return;
    }

    let new_ship = match ship_type {
        "fly" => Ship::new_fly(),
        "bullet" => Ship::new_bullet(true, 1),
        "explosion" => Ship::new_explosion(),
        _ => return,
    };

    grid.insert(position, new_ship);
}

pub fn move_entity(
    old_coords: Cords,
    new_coords: Cords,
    grid: &mut HashMap<Cords, Ship>,
) -> Result<Option<String>, &'static str> {
    if let Some(entity) = grid.remove(&Cords(old_coords.0, old_coords.1)) {
        let is_bullet = matches!(entity, Ship::Bullet(_, _, _));

        if grid.contains_key(&Cords(new_coords.0, new_coords.1)) {
            if is_bullet {
                let existing_ship = grid.remove(&Cords(new_coords.0, new_coords.1));
                let removed_type = if let Some(ship) = &existing_ship {
                    Some(ship.display_type().to_string())
                } else {
                    None
                };

                if existing_ship.is_some() {
                    grid.insert(Cords(new_coords.0, new_coords.1), Ship::new_explosion());
                }

                return Ok(removed_type);
            } else {
                grid.insert(Cords(old_coords.0, old_coords.1), entity);
                return Err("Target position is already in use.");
            }
        } else {
            grid.insert(Cords(new_coords.0, new_coords.1), entity);
            return Ok(None);
        }
    } else {
        Err("No entity found at the given old coordinates.")
    }
}

pub fn ship_actions(
    grid: &mut HashMap<Cords, Ship>,
    rows: u32,
    cols: u32,
    settings: &mut GameSettings,
    score: &Arc<Mutex<u32>>,
) {
    let half_rows = (rows / 2) as usize;
    let entries: Vec<(Cords, Uuid)> = grid
        .iter()
        .map(|(&coords, ship)| (coords, ship.get_id()))
        .collect();

    let mut coords_and_actions: Vec<(Cords, ShipAction)> = Vec::new();

    for (coords, _) in &entries {
        if let Some(mut ship) = grid.remove(coords) {
            let action = ship.get_action(*coords, &*grid);
            grid.insert(*coords, ship);
            coords_and_actions.push((*coords, action));
        }
    }

    let mut bullet_actions = Vec::new();
    let mut other_actions = Vec::new();

    for (coords, action) in coords_and_actions {
        if let Some(ship) = grid.get(&coords) {
            let is_bullet = matches!(ship, Ship::Bullet(_, _, _));

            if is_bullet {
                bullet_actions.push((coords, action));
            } else {
                other_actions.push((coords, action));
            }
        }
    }

    for (coords, action) in bullet_actions {
        match action {
            ShipAction::Move(new_coords, _) => {

                if new_coords.0 >= rows as usize || new_coords.1 >= cols as usize {
                    grid.remove(&coords);
                    continue;
                }

                let result = move_entity(coords, new_coords, grid);
                if let Ok(Some(removed_type)) = result {
                    if removed_type == "fly" {
                        let mut score_guard = score.lock().unwrap();
                        *score_guard += 100;
                    }
                }
            }
            ShipAction::Remove => {
                grid.remove(&coords);
            }
            _ => {}
        }
    }

    if settings.can_flies_move() {
        for (coords, action) in other_actions {
            if !grid.contains_key(&coords) {
                continue;
            }

            match action {
                ShipAction::Move(new_coords, _) => {

                    if new_coords.0 < half_rows &&
                        new_coords.0 < rows as usize &&
                        new_coords.1 < cols as usize {
                        let result = move_entity(coords, new_coords, grid);
                        if let Ok(Some(removed_type)) = result {
                            if removed_type == "fly" {
                                let mut score_guard = score.lock().unwrap();
                                *score_guard += 100;
                            }
                        }
                    } else {
                        println!("Skipping move: out-of-bound or below half grid movement blocked");
                    }
                }
                ShipAction::Shoot => {
                    if settings.can_shoot(false) {
                        let bullet_coords = Cords(coords.0 + 1, coords.1);
                        if bullet_coords.0 < rows as usize && bullet_coords.1 < cols as usize &&
                            !grid.contains_key(&bullet_coords) {
                            let bullet_speed = 11 - settings.laser_speed;
                            let mut bullet = Ship::new_bullet(true, bullet_speed as u64);

                            if let Ship::Bullet(ref mut ai, _, _) = bullet {
                                let speed_factor = 11 - settings.laser_speed;
                                ai.timer = Timer::new(speed_factor as u64);
                            }

                            grid.insert(bullet_coords, bullet);
                        }
                    }
                }
                ShipAction::Remove => {
                    if let Some(ship) = grid.get(&coords) {
                        if ship.display_type() == "fly" {
                            let mut score_guard = score.lock().unwrap();
                            *score_guard += 100;
                        }
                    }
                    grid.remove(&coords);
                }
                ShipAction::Nothing => {
                }
            }
        }
    }

    let final_cleanup: Vec<Cords> = grid
        .iter()
        .filter_map(|(&coords, ship)| {
            match ship {
                Ship::Bullet(_, _, _) => {
                    if coords.0 >= rows as usize || coords.1 >= cols as usize {
                        Some(coords)
                    } else {
                        None
                    }
                },
                _ => None
            }
        })
        .collect();

    for coords in final_cleanup {
        grid.remove(&coords);
    }
}
