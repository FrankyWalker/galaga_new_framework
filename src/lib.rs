use rand::Rng;
use rust_on_rails::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::button_struct::Button;
use crate::player::{integrate_player_to_game, Player};
use crate::settings::{ensure_flies_on_grid, spawn_initial_flies, GameSettings};
use crate::ship::Ship;
use crate::ship_actions::ship_actions;
use crate::structs::{Cords, COLUMNS, ROWS};
use process_message::process_message;
use refresh_display::refresh_display;
use server::run_server;
use server::PressurePadData;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::{interval, Duration};
use warp::query;

mod structs;
mod ship;
mod player;
mod settings;
mod ship_actions;
mod server;
mod process_message;
mod button_struct;
mod refresh_display;

const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

#[derive(Clone)]
struct Star {
    x: u32,
    y: f32,
    speed: f32,
    color: &'static str,
    size: (u32, u32),
}

pub struct MyApp {
    grid: HashMap<Cords, Ship>,
    items: Vec<CanvasItem>,
    rows: usize,
    cols: usize,
    cell_size: (u32, u32),
    margin: u32,
    player: Player,
    settings: GameSettings,
    window_size: (u32, u32),
    buttons: Vec<Button>,
    stars: Vec<Star>,
    image_fly: ImageKey,
    explosion: ImageKey,
    bullet_downward: ImageKey,
    bullet_upward: ImageKey,
    player_image: ImageKey,
    score: u32,
    last_update: std::time::Instant,
    rx_arc: Arc<TokioMutex<mpsc::Receiver<PressurePadData>>>,
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

        let score = 0;
        let mut grid = HashMap::new();
        let mut items = Vec::new();
        let mut player = Player::new();
        let mut settings = GameSettings::new();

        ensure_flies_on_grid(&settings, &mut grid, rows as u32, cols as u32);

        let image_fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let explosion = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());
        let bullet_downward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_downward.png")).unwrap().into());
        let bullet_upward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_upward.png")).unwrap().into());
        let player_image = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/spaceship.png")).unwrap().into());

        let mut buttons = Vec::new();
        buttons.push(Button::new(|| println!("Fly Speed -"), (200, 40), (30, 910), "Fly Speed +"));
        buttons.push(Button::new(|| println!("Fly Speed +"), (200, 40), (30, 850), "Fly Speed -"));

        buttons.push(Button::new(|| println!("Laser Speed -"), (200, 40), (190, 910), "Laser Speed +"));
        buttons.push(Button::new(|| println!("Laser Speed +"), (200, 40), (190, 850), "Laser Speed -"));

        buttons.push(Button::new(|| println!("Flies -"), (200, 40), (380, 910), "Flies -"));
        buttons.push(Button::new(|| println!("Flies +"), (200, 40), (380, 850), "Flies +"));

        buttons.push(Button::new(|| println!("Invincible"), (200, 40), (480, 910), "Invincible"));
        buttons.push(Button::new(|| println!("Fly Move"), (200, 40), (480, 850), "Fly Move"));

        buttons.push(Button::new(|| println!("Laser Shoot"), (200, 40), (615, 910), "Lasers"));
        buttons.push(Button::new(|| println!("Reset"), (200, 40), (610, 850), "Save & Restart"));

        spawn_initial_flies(
            &settings,
            &mut grid,
            rows as u32,
            cols as u32
        );

        println!("yeet222222");
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

        println!("yeetiie3");
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
            &stars,
            image_fly,
            explosion,
            bullet_downward,
            bullet_upward,
            player_image,
        );

        

        MyApp {
            grid,
            items,
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
            last_update: std::time::Instant::now(),
            rx_arc
        }
    }


    async fn draw(&mut self, ctx: &mut Context) {
        println!("running draw function");


            let rx_arc = self.rx_arc.clone();


            process_message(
                rx_arc,
                &mut self.player,
                &mut self.settings,
                &mut self.grid,
            ).await;


            ship_actions(&mut self.grid, self.rows as u32, self.cols as u32, &mut self.settings, self.score);

            integrate_player_to_game(
                &mut self.player,
                &mut self.grid,
                &mut self.items,
                self.cell_size,
                self.margin
            );

            if let Some(pos) = self.player.current_position {
                if self.grid.contains_key(&pos) {
                    if !self.settings.invincible {
                        if let Some(remaining_lives) = self.player.handle_collision() {
                            if remaining_lives == 5 {
                                self.grid.clear();
                                spawn_initial_flies(
                                    &self.settings,
                                    &mut self.grid,
                                    self.rows as u32,
                                    self.cols as u32,
                                );
                            }

                            if let Some(ship) = self.grid.get(&pos) {
                                if ship.display_type() == "fly" {
                                    self.score += 100;
                                }
                            }
                            self.grid.remove(&pos);
                        }
                    } else {
                        if let Some(ship) = self.grid.get(&pos) {
                            if ship.display_type() == "fly" {
                                self.score += 100;
                            }
                        }
                        self.grid.remove(&pos);
                    }
                }
            }

            refresh_display(
                &mut self.grid,
                &mut self.items,
                &mut self.player,
                &self.settings,
                self.rows as u32,
                self.cols as u32,
                self.cell_size,
                self.margin,
                self.window_size,
                0,
                &self.stars,
                self.image_fly,
                self.explosion,
                self.bullet_downward,
                self.bullet_upward,
                self.player_image,
            );

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
            }


        ctx.clear("000000");



        for item in &self.items {
            if let CanvasItem::Shape(_, _, _, _) = item {
                ctx.draw(*item);
            }
        }
        for item in &self.items {
            if let CanvasItem::Image(_, _, _) = item {
                ctx.draw(*item);
            }
        }

        for star in &self.stars {
            ctx.draw(CanvasItem::Shape(
                Area((star.x, star.y as u32), None),
                Shape::Rectangle(0, star.size),
                star.color,
                255,
            ));
        }

        // ctx.draw(CanvasItem::Shape(
        //     Area((9, 760), None),
        //     Shape::RoundedRectangle(0, (805, 220), 10),
        //     "0D1F2D",
        //     255
        // ));


        // for button in &self.buttons {
        //     button.return_canvas_item(ctx);
        // }
        //
        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

        ctx.draw(CanvasItem::Text(
            Area((20, 20), None),
            Text::new(
                format!("Score: {}", self.score).leak(),
                "FF0000",
                255,
                Some(800),
                25,
                30,
                font
            )
        ));

        // let text_color = "FFFFFF";
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((30, 780), None),
        //     Text::new(
        //         format!("Fly Speed: {}", self.settings.get_fly_speed()).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((310, 780), None),
        //     Text::new(
        //         format!("Laser Speed: {}", self.settings.laser_speed).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((570, 780), None),
        //     Text::new(
        //         format!("Flies: {}", self.settings.number_of_flies).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((570, 810), None),
        //     Text::new(
        //         format!("Invincible: {}", if self.settings.invincible { "ON" } else { "OFF" }).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((30, 810), None),
        //     Text::new(
        //         format!("Fly Movement: {}", if self.settings.fly_move { "ON" } else { "OFF" }).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
        //
        // ctx.draw(CanvasItem::Text(
        //     Area((310, 810), None),
        //     Text::new(
        //         format!("Laser Shoot: {}", if self.settings.laser_shoot { "ON" } else { "OFF" }).leak(),
        //         text_color,
        //         255,
        //         Some(800),
        //         20,
        //         25,
        //         font
        //     )
        // ));
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        let position = ctx.position;
        let mut player = &mut self.player;
        let mut settings = &mut self.settings;
        let mut grid = &mut self.grid;

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
