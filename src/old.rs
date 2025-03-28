use rust_on_rails::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, LazyLock};
use rand::Rng;
use tokio::time::{interval, Duration};

use crate::player::Player;
use crate::ship::Ship;
use crate::structs::{Cords, ShipAction, COLUMNS, ROWS};

mod structs;
mod ship;
mod player;

pub struct MyApp {
    pub player: Player,
    grid: Arc<Mutex<HashMap<(u32, u32), Ship>>>,
    items: Arc<Mutex<Vec<CanvasItem>>>,
    font: FontKey,
    rows: usize,
    cols: usize,
    cell_size: (u32, u32),
    margin: u32,
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let rows = ROWS as usize;
        let cols = COLUMNS as usize;
        let cell_size = (50, 50);
        let margin = 5;
        let grid = Arc::new(Mutex::new(HashMap::new()));
        let items = Arc::new(Mutex::new(Vec::new()));

        let player = Player::new();

        {
            let mut grid_lock = grid.lock().unwrap();

            add_ship(Ship::new_fly(), 2, 2, &mut grid_lock);
            add_ship(Ship::new_fly(), 3, 3, &mut grid_lock);
            add_ship(Ship::new_fly(), 1, 4, &mut grid_lock);

            let mut items_lock = items.lock().unwrap();
            refresh_display(&grid_lock, &mut items_lock, rows as u32, cols as u32, cell_size, margin);
        }

        let app = MyApp {
            player,
            grid: Arc::clone(&grid),
            items: Arc::clone(&items),
            font: 0,
            rows,
            cols,
            cell_size,
            margin,
        };

        let grid_clone = Arc::clone(&grid);
        let items_clone = Arc::clone(&items);

        let app_state = Arc::new(Mutex::new(AppState {
            rows: rows as u32,
            cols: cols as u32,
            cell_size,
            margin,
            player: Player::new(),
        }));

        let app_state_clone = Arc::clone(&app_state);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(500));

            loop {
                interval.tick().await;

                let app_state = app_state_clone.lock().unwrap();
                let mut grid = grid_clone.lock().unwrap();
                let mut items = items_clone.lock().unwrap();

                ship_actions(&app_state, &mut grid);

                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.2) {
                    let col = rng.gen_range(0..app_state.cols);
                    add_ship(Ship::new_fly(), 0, col, &mut grid);
                }

                move_bullets(&app_state, &mut grid);
                handle_collisions(&app_state, &mut grid);
                process_explosions(&mut grid);

                refresh_display(
                    &grid,
                    &mut items,
                    app_state.rows,
                    app_state.cols,
                    app_state.cell_size,
                    app_state.margin
                );
            }
        });

        app
    }

    async fn draw(&mut self, ctx: &mut Context) {
        ctx.clear("000000");

        if let Some(pos) = self.player.current_position {
            let x = pos.1 as u32 * (self.cell_size.0 + self.margin);
            let y = pos.0 as u32 * (self.cell_size.1 + self.margin);

            ctx.draw(CanvasItem::Shape(
                Area((x, y), None),
                Shape::RoundedRectangle(0, self.cell_size, 10),
                "00FF00",
                255,
            ));
        }

        let items = self.items.lock().unwrap();
        for item in items.iter() {
            ctx.draw(*item);
        }
    }

    async fn on_click(&mut self, ctx: &mut Context) {}

    async fn on_move(&mut self, ctx: &mut Context) {}

    async fn on_press(&mut self, _ctx: &mut Context, _key: String) {}
}

#[derive(Clone)]
struct AppState {
    rows: u32,
    cols: u32,
    cell_size: (u32, u32),
    margin: u32,
    player: Player,
}

fn add_ship(
    ship: Ship,
    row: u32,
    col: u32,
    grid: &mut HashMap<(u32, u32), Ship>,
) {
    let position = (row, col);
    if grid.contains_key(&position) {
        return;
    }
    grid.insert(position, ship);
}

fn move_entity(
    old_coords: (u32, u32),
    new_coords: (u32, u32),
    grid: &mut MutexGuard<HashMap<(u32, u32), Ship>>,
) -> Result<(), &'static str> {
    if let Some(entity) = grid.remove(&old_coords) {
        if !grid.contains_key(&new_coords) {
            grid.insert(new_coords, entity);
            Ok(())
        } else {
            grid.insert(old_coords, entity);
            Err("Target position is already in use.")
        }
    } else {
        Err("No entity found at the given old coordinates.")
    }
}

fn refresh_display(
    grid: &HashMap<(u32, u32), Ship>,
    items: &mut Vec<CanvasItem>,
    rows: u32,
    cols: u32,
    cell_size: (u32, u32),
    margin: u32,
) {
    items.clear();
    let background_color = "149414";
    let static_color = Box::leak(background_color.to_string().into_boxed_str());

    for row in 0..rows {
        for col in 0..cols {
            let x = col * (cell_size.0 + margin);
            let y = row * (cell_size.1 + margin);
            items.push(CanvasItem::Shape(
                Area((x, y), None),
                Shape::RoundedRectangle(0, cell_size, 10),
                static_color,
                255,
            ));
        }
    }

    for (&(row, col), ship) in grid.iter() {
        let x = col * (cell_size.0 + margin);
        let y = row * (cell_size.1 + margin);

        let color = match ship {
            Ship::Fly(_, _, _) => "FF0000",
            Ship::Bullet(_, _, _) => "0000FF",
            Ship::Explosion(_, _, _) => "FFA500",
        };

        items.push(CanvasItem::Shape(
            Area((x, y), None),
            Shape::RoundedRectangle(0, cell_size, 10),
            color,
            255,
        ));
    }
}

fn ship_actions(
    app_state: &AppState,
    grid: &mut MutexGuard<HashMap<(u32, u32), Ship>>,
) {
    let mut actions_to_process = Vec::new();
    let mut ships_to_remove = Vec::new();

    let ship_positions: Vec<(u32, u32)> = grid.keys().cloned().collect();

    for coords in ship_positions {
        if let Some(ship) = grid.get_mut(&coords) {
            let mut ship_clone = ship.clone();
            let action = ship_clone.get_action(coords, grid);
            actions_to_process.push((coords, action));
        }
    }

    for (coords, action) in actions_to_process {
        match action {
            ship::ShipAction::Move(new_coords, _) => {
                if new_coords.0 < app_state.rows as usize as u32 && new_coords.1 < app_state.cols as usize as u32 {
                    let key = (new_coords.0 as u32, new_coords.1 as u32);
                    if !grid.contains_key(&key) {
                        if let Err(_) = move_entity(coords, key, grid) {
                            ships_to_remove.push(coords);
                        }
                    } else {
                        ships_to_remove.push(coords);
                    }
                } else {
                    ships_to_remove.push(coords);
                }
            },
            ship::ShipAction::Shoot => {
                if let Some(ship) = grid.get(&coords) {
                    match ship {
                        Ship::Fly(_, _, _) => {
                            let bullet_coords = (coords.0 + 1, coords.1);
                            if bullet_coords.0 < app_state.rows && !grid.contains_key(&bullet_coords) {
                                grid.insert(bullet_coords, Ship::new_bullet(true));
                            }
                        },
                        _ => {}
                    }
                }
            },
            ship::ShipAction::Remove => {
                ships_to_remove.push(coords);
            },
            ship::ShipAction::Nothing => {},
        }
    }

    for coords in ships_to_remove {
        grid.remove(&coords);
    }
}
fn move_bullets(
    app_state: &AppState,
    grid: &mut MutexGuard<HashMap<(u32, u32), Ship>>,
) {

}

fn handle_collisions(
    app_state: &AppState,
    grid: &mut MutexGuard<HashMap<(u32, u32), Ship>>,
) {
    let mut collisions = Vec::new();

    let positions: Vec<(u32, u32)> = grid.keys().cloned().collect();

    for &pos in &positions {
        if let Some(ship) = grid.get(&pos) {
            match ship {
                Ship::Fly(_, _, _) => {

                    for &other_pos in &positions {
                        if pos == other_pos {
                            continue;
                        }

                        if let Some(other_ship) = grid.get(&other_pos) {
                            match other_ship {
                                Ship::Bullet(_, _, _) => {
                                    if pos == other_pos {
                                        collisions.push(pos);
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    for pos in collisions {
        for &coords in &positions {
            if coords == pos {
                grid.remove(&coords);
            }
        }
        grid.insert(pos, Ship::new_explosion());
    }
}

fn process_explosions(
    grid: &mut MutexGuard<HashMap<(u32, u32), Ship>>,
) {

}

create_entry_points!(MyApp);

