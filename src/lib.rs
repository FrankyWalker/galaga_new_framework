use rand::Rng;
use rust_on_rails::prelude::*;
use std::sync::Arc;

use crate::fly_spawner::FlySpawner;
use crate::game_image_handler::GameImageHandler;
use crate::game_renderer::GameRenderer;
use crate::message_processor::MessageProcessor;
use crate::player::Player;
use crate::settings::{Settings, ButtonAction};
use crate::ship::ShipGrid;
use crate::star_background::StarBackground;

use prelude::App;
use server::run_server;
use server::PressurePadData;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;

mod structs;
mod ship;
mod player;
mod settings;
mod server;
mod game_image_handler;
mod message_processor;
mod game_renderer;
mod fly_spawner;
mod star_background;
mod ship_ai;

const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

pub struct MyApp {
    window_size: (u32, u32),
    images: GameImageHandler,
    rx_arc: Arc<TokioMutex<mpsc::Receiver<PressurePadData>>>,
    ship_grid: ShipGrid,
    player: Player,
    settings: Settings,
    message_processor: MessageProcessor,
    game_renderer: GameRenderer,
    fly_spawner: FlySpawner,
    star_background: Option<StarBackground>,
    font: FontKey,
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let (tx, rx) = mpsc::channel::<PressurePadData>(100);
        let rx_arc = Arc::new(TokioMutex::new(rx));

        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);
        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

        tokio::spawn(async move {
            run_server(tx).await;
        });

        let settings = Settings::new();

        let pressure_threshold = 600;
        let mut ship_grid = ShipGrid::new();
        let mut fly_spawner = FlySpawner::new();
        let message_processor = MessageProcessor::new(pressure_threshold);
        let game_renderer = GameRenderer::new();
        let star_background = Some(StarBackground::new(window_size));
        let player = Player::new();
        ship_grid.grid = fly_spawner.spawn_flies(settings.value_stats.number_of_flies.clone());

        let images = GameImageHandler::new(ctx);

        MyApp {
            window_size,
            images,
            rx_arc,
            ship_grid,
            player,
            settings,
            message_processor,
            game_renderer,
            fly_spawner,
            star_background,
            font,
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        self.process_game_state();

        ctx.clear("000000");
        if let Some(star_background) = &self.star_background {
            star_background.draw(ctx);
        }

        self.game_renderer.draw(
            ctx,
            &self.ship_grid.grid,
            &self.player,
            &self.images,
            self.get_score(),
            self.font,
        );

        ctx.draw(CanvasItem::Shape(
            Area((9, 760), None),
            Shape::RoundedRectangle(0, (805, 220), 10),
            "0D1F2D",
            255
        ));

        self.settings.draw(ctx, self.font);
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        let position = ctx.position;
        if let Some(action) = self.settings.handle_click(position.0, position.1) {
            if matches!(action, ButtonAction::Reset) {
                self.spawn_initial_flies();
                self.fly_spawner = FlySpawner::new();
            }
        }
    }

    async fn on_move(&mut self, _ctx: &mut Context) {}

    async fn on_press(&mut self, _ctx: &mut Context, _t: String) {}
}

impl MyApp {
    fn get_score(&self) -> u32 {
        self.ship_grid.score
    }


    fn process_game_state(&mut self) {
        self.player.update();
        self.player.shoot(&mut self.ship_grid.grid);
        self.handle_player_actions();

        if let Some(star_background) = &mut self.star_background {
            star_background.on_tick();
        }

        self.ship_grid.process_ship_actions(&self.settings);

        self.check_for_level_completion();
    }

    fn check_for_level_completion(&mut self) {
        let fly_count = self.ship_grid.grid.iter().filter(|(_, ship)| {
            let ship_type = ship.display_type();
            ship_type == "fly" || ship_type == "tiki_fly" ||
                ship_type == "northrop_fly" || ship_type == "b2_fly"
        }).count();

        let explosion_count = self.ship_grid.grid.iter()
            .filter(|(_, ship)| ship.display_type() == "explosion")
            .count();

        if fly_count == 0 && explosion_count == 0 {
            self.ship_grid.grid = self.fly_spawner.spawn_next_level();
        }
    }

    fn spawn_initial_flies(&mut self) {
        self.fly_spawner.reset_level();
        self.ship_grid.clear();
        self.ship_grid.grid = self.fly_spawner.spawn_flies(self.fly_spawner.get_current_fly_count());
    }

    fn handle_player_actions(&mut self) -> bool {
        let mut player_died = false;

        if let Some(pos) = self.player.current_position {
            player_died = self.player.handle_collision(
                &mut self.ship_grid.grid,
                pos,
                &mut self.ship_grid.score,
                self.settings.value_stats.invincible);

            if player_died {
                self.spawn_initial_flies();
            }
        }

        player_died
    }
}

create_entry_points!(MyApp);
