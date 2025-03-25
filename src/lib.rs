use rand::Rng;
use rust_on_rails::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::button_struct::Button;
use crate::player::{integrate_player_to_game, Player};
use crate::settings::{create_game_settings, ensure_flies_on_grid, spawn_initial_flies, GameSettings};
use crate::ship::Ship;
use crate::ship_actions::ship_actions;
use crate::structs::{Cords, COLUMNS, ROWS};
use process_messages::process_messages;
use refresh_display::refresh_display;
use server::run_server;
use server::PressurePadData;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{interval, Duration};

mod structs;
mod ship;
mod player;
mod settings;
mod ship_actions;
mod server;
mod process_messages;
mod button_struct;
mod refresh_display;

const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1000;

#[derive(Clone)]
struct Star {
    x: u32,
    y: f32,
    speed: f32,
    color: &'static str,
    size: (u32, u32),
}

pub struct MyApp {
    grid: Arc<Mutex<HashMap<Cords, Ship>>>,
    items: Arc<Mutex<Vec<CanvasItem>>>,
    font: FontKey,
    rows: usize,
    cols: usize,
    cell_size: (u32, u32),
    margin: u32,
    player: Arc<Mutex<Player>>,
    settings: Arc<Mutex<GameSettings>>,
    window_size: (u32, u32),
    buttons: Vec<Button>,
    stars: Vec<Star>,
    image_fly: ImageKey,
    explosion: ImageKey,
    bullet_downward: ImageKey,
    bullet_upward: ImageKey,
    player_image: ImageKey,
    score: Arc<Mutex<u32>>,
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let (tx, rx) = mpsc::channel::<PressurePadData>(100);

        let rx_arc = Arc::new(TokioMutex::new(rx));

        tokio::spawn(async move {
            run_server(tx).await;
        });

        let rows = ROWS;
        let cols = COLUMNS;
        let cell_size = (50, 50);
        let margin = 5;

        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);

        let score = Arc::new(Mutex::new(0));
        let grid = Arc::new(Mutex::new(HashMap::new()));
        let items = Arc::new(Mutex::new(Vec::new()));
        let player = Arc::new(Mutex::new(Player::new()));
        let settings = create_game_settings();

        ensure_flies_on_grid(&settings.lock().unwrap(), &mut grid.lock().unwrap(), rows as u32, cols as u32);

        let image_fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let explosion = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());
        let bullet_downward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_downward.png")).unwrap().into());
        let bullet_upward =  ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_upward.png")).unwrap().into());
        let player_image = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/spaceship.png")).unwrap().into());

        let rx_arc_clone = Arc::clone(&rx_arc);
        let player_clone_for_messages = Arc::clone(&player);
        let settings_clone_for_messages = Arc::clone(&settings);
        let grid_clone_for_messages = Arc::clone(&grid);

        tokio::spawn(async move {
            process_messages(
                rx_arc_clone,
                player_clone_for_messages,
                settings_clone_for_messages,
                grid_clone_for_messages
            ).await;
        });

        let mut buttons = Vec::new();
        buttons.push(Button::new(|| println!("Fly Speed -"), (100, 40), (30, 910), "Fly Speed +"));
        buttons.push(Button::new(|| println!("Fly Speed +"), (100, 40), (30, 850), "Fly Speed -"));

        buttons.push(Button::new(|| println!("Laser Speed -"), (100, 40), (190, 910), "Laser Speed +"));
        buttons.push(Button::new(|| println!("Laser Speed +"), (100, 40), (190, 850), "Laser Speed -"));

        buttons.push(Button::new(|| println!("Flies -"), (100, 40), (380, 910), "Flies +"));
        buttons.push(Button::new(|| println!("Flies +"), (100, 40), (380, 850), "Flies -"));

        buttons.push(Button::new(|| println!("Invincible"), (100, 40), (480, 910), "Invincible"));
        buttons.push(Button::new(|| println!("Fly Move"), (100, 40), (480, 850), "Fly Move"));

        buttons.push(Button::new(|| println!("Laser Shoot"), (100, 40), (615, 910), "Lasers"));
        buttons.push(Button::new(|| println!("Reset"), (100, 40), (610, 850), "Save & Restart"));

        let grid_clone = Arc::clone(&grid);
        let items_clone = Arc::clone(&items);
        let player_clone = Arc::clone(&player);
        let settings_clone = Arc::clone(&settings);

        spawn_initial_flies(
            &settings_clone.lock().unwrap(),
            &mut grid_clone.lock().unwrap(),
            rows as u32,
            cols as u32
        );

        let mut stars = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..150 {
            let color = match rng.gen_range(0..4) {
                0 => "0099FF",
                1 => "FF6600",
                2 => "FF0000",
                _ => "FFFF00",
            };

            stars.push(Star {
                x: rng.gen_range(0..window_size.0),
                y: rng.gen_range(0.0..window_size.1 as f32),
                speed: rng.gen_range(0.5..3.0),
                color,
                size: (rng.gen_range(1..4), rng.gen_range(1..4)),
            });
        }

        //let score_clone1 = Arc::clone(&score);

        refresh_display(
            &mut grid_clone.lock().unwrap(),
            &mut items_clone.lock().unwrap(),
            &mut player_clone.lock().unwrap(),
            &settings_clone.lock().unwrap(),
            rows as u32,
            cols as u32,
            cell_size,
            margin,
            window_size,
            0,
            &stars,
            image_fly,
            explosion,
            bullet_downward,
            bullet_upward,
            player_image,
            //&score_clone1
        );


        let grid_clone = Arc::clone(&grid);
        let items_clone = Arc::clone(&items);
        let player_clone = Arc::clone(&player);
        let settings_clone = Arc::clone(&settings);
        let stars_clone = stars.clone();

        let score_clone2 = Arc::clone(&score);

        let image_fly_clone = image_fly;
        let explosion_clone = explosion;
        let bullet_downward_clone = bullet_downward;
        let bullet_upward_clone = bullet_upward;
        let player_image_clone = player_image;

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(50));

            loop {
                interval.tick().await;
                let mut grid = grid_clone.lock().unwrap();
                let mut items = items_clone.lock().unwrap();
                let mut player = player_clone.lock().unwrap();
                let mut settings = settings_clone.lock().unwrap();

                ship_actions(&mut grid, rows as u32, cols as u32, &mut settings, &score_clone2);

                integrate_player_to_game(&mut player, &mut grid, &mut items, cell_size, margin);

                if let Some(pos) = player.current_position {
                    if grid.contains_key(&pos) {
                        if !settings.invincible {
                            if let Some(remaining_lives) = player.handle_collision() {
                                if remaining_lives == 5 {
                                    grid.clear();
                                    spawn_initial_flies(
                                        &settings,
                                        &mut grid,
                                        rows as u32,
                                        cols as u32,
                                    );
                                }

                                if let Some(ship) = grid.get(&pos) {
                                    if ship.display_type() == "fly" {
                                        let mut score_guard = score_clone2.lock().unwrap();
                                        *score_guard += 100;
                                    }
                                }
                                grid.remove(&pos);
                            }
                        } else {
                            if let Some(ship) = grid.get(&pos) {
                                if ship.display_type() == "fly" {
                                    let mut score_guard = score_clone2.lock().unwrap();
                                    *score_guard += 100;
                                }
                            }
                            grid.remove(&pos);
                        }
                    }
                }


                refresh_display(
                    &mut grid,
                    &mut items,
                    &mut player,
                    &settings,
                    rows as u32,
                    cols as u32,
                    cell_size,
                    margin,
                    window_size,
                    0,
                    &stars_clone,
                    image_fly_clone,
                    explosion_clone,
                    bullet_downward_clone,
                    bullet_upward_clone,
                    player_image_clone,
                    //&score_clone2
                );

            }
        });

        MyApp {
            grid,
            items,
            font: 0,
            rows,
            cols,
            cell_size,
            margin,
            player,
            settings,
            window_size,
            buttons,
            stars,
            image_fly,
            explosion,
            bullet_downward,
            bullet_upward,
            player_image,
            score,
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        ctx.clear("000000");

        let items = self.items.lock().unwrap();
        for item in items.iter() {
            if let CanvasItem::Shape(_, _, _, _) = item {
                ctx.draw(*item);
            }
        }

        let mut rng = rand::thread_rng();
        for star in &mut self.stars {
            star.y += star.speed;

            if star.y > self.window_size.1 as f32 {
                star.y = 0.0;
                star.x = rng.gen_range(0..self.window_size.0);
                star.speed = rng.gen_range(0.5..3.0);

                if rng.gen_bool(0.3) {
                    star.color = match rng.gen_range(0..4) {
                        0 => "0099FF",
                        1 => "FF6600",
                        2 => "FF0000",
                        _ => "FFFF00",
                    };
                }
            }

            ctx.draw(CanvasItem::Shape(
                Area((star.x, star.y as u32), None),
                Shape::Rectangle(0, star.size),
                star.color,
                255,
            ));
        }

        for item in items.iter() {
            if let CanvasItem::Image(_, _, _) = item {
                ctx.draw(*item);
            }
        }

        ctx.draw(CanvasItem::Shape(
            Area((9, 760), None),
            Shape::RoundedRectangle(0, (805, 220), 10),
            "0D1F2D",
            255
        ));

        for button in &self.buttons {
            button.return_canvas_item(ctx);
        }

        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

        // Score display
        ctx.draw(CanvasItem::Text(
            Area((20, 20), None),
            Text::new(
                format!("Score: {}", self.score.lock().unwrap()).leak(),
                "FF0000",
                255,
                Some(800),
                25,
                30,
                font
            )
        ));

        let settings = self.settings.lock().unwrap();
        let text_color = "FFFFFF";

        ctx.draw(CanvasItem::Text(
            Area((30, 780), None),
            Text::new(
                format!("Fly Speed: {}", settings.get_fly_speed()).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));

        ctx.draw(CanvasItem::Text(
            Area((310, 780), None),
            Text::new(
                format!("Laser Speed: {}", settings.laser_speed).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));

        ctx.draw(CanvasItem::Text(
            Area((570, 780), None),
            Text::new(
                format!("Flies: {}", settings.number_of_flies).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));

        ctx.draw(CanvasItem::Text(
            Area((570, 810), None),
            Text::new(
                format!("Invincible: {}", if settings.invincible { "ON" } else { "OFF" }).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));

        // Third Row
        ctx.draw(CanvasItem::Text(
            Area((30, 810), None),
            Text::new(
                format!("Fly Movement: {}", if settings.fly_move { "ON" } else { "OFF" }).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));

        ctx.draw(CanvasItem::Text(
            Area((310, 810), None),
            Text::new(
                format!("Laser Shoot: {}", if settings.laser_shoot { "ON" } else { "OFF" }).leak(),
                text_color,
                255,
                Some(800),
                20,
                25,
                font
            )
        ));
    }
    async fn on_click(&mut self, ctx: &mut Context) {
        let position = ctx.position;
        let mut player = self.player.lock().unwrap();
        let mut settings = self.settings.lock().unwrap();
        let mut grid = self.grid.lock().unwrap();

        for (index, button) in self.buttons.iter_mut().enumerate() {
            let was_clicked = button.is_within_bounds(position.0, position.1);
            if was_clicked {
                for (index, button) in self.buttons.iter_mut().enumerate() {
                    let was_clicked = button.is_within_bounds(position.0, position.1);
                    if was_clicked {
                        match index {
                            0 => {
                                let current_fly_speed = settings.get_fly_speed();
                                let new_fly_speed = settings.set_fly_speed(current_fly_speed.saturating_sub(1));
                                println!("Fly Speed reduced to: {}", new_fly_speed);
                            }
                            1 => {
                                let current_fly_speed = settings.get_fly_speed();
                                let new_fly_speed = settings.set_fly_speed(current_fly_speed.saturating_add(1));
                                println!("Fly Speed increased to: {}", new_fly_speed);
                            }
                            2 => {
                                let laser_speed = settings.laser_speed;
                                settings.set_laser_speed(laser_speed.saturating_add(1));
                            }
                            3 => {
                                let laser_speed = settings.laser_speed;
                                settings.set_laser_speed(laser_speed.saturating_sub(1));
                            }
                            4 => {
                                let number_of_flies = settings.number_of_flies;
                                settings.set_number_of_flies(number_of_flies.saturating_sub(1));
                            }
                            5 => {
                                let number_of_flies = settings.number_of_flies;
                                settings.set_number_of_flies(number_of_flies.saturating_add(1));
                            }
                            6 => {
                                let invincible = settings.invincible;
                                settings.set_invincible(!invincible);
                            }
                            7 => {
                                let fly_move = settings.fly_move;
                                settings.set_fly_movement(!fly_move);
                            }
                            8 => {
                                let laser_shoot = settings.laser_shoot;
                                settings.set_laser_shooting(!laser_shoot);
                            }
                            9 => {
                                spawn_initial_flies(
                                    &settings,
                                    &mut grid,
                                    self.rows as u32,
                                    self.cols as u32,
                                );
                            }
                            _ => {}
                        }
                        break;
                    }
                }
                break;
            }
        }
    }

    async fn on_move(&mut self, _ctx: &mut Context) {}

    async fn on_press(&mut self, ctx: &mut Context, t: String) {
    }
}


pub struct ScoreDisplay(pub String);


create_entry_points!(MyApp);
