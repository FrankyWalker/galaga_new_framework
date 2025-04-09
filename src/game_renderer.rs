use crate::structs::Cords;
use crate::player::Player;
use crate::ship::{Ship, };
use crate::game_image_handler::GameImageHandler;
use rust_on_rails::prelude::*;
use std::collections::HashMap;
use crate::ship_ai::AIAction;
use crate::structs::{MARGIN, ROWS, START_X, START_Y, CELL_SIZE};

pub struct GameRenderer;

//GameRenderer returns canvas items like the fly bullets ETC this gets called in Lib.rs.

impl GameRenderer {
    pub fn new() -> Self {
        GameRenderer
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        grid: &HashMap<Cords, Ship>,
        player: &Player,
        image_handler: &GameImageHandler,
        score: u32,
        font: FontKey,
    ) {
        let canvas_items = self.get_game_canvas_items(grid, player, image_handler);
        for item in canvas_items {
            ctx.draw(item);
        }

        ctx.draw(CanvasItem::Text(
            Area((20, 20), None),
            Text::new(
                format!("Score: {}", score).leak(),
                "FF0000",
                255,
                Some(800),
                25,
                30,
                font
            )
        ));

    }

    pub fn get_game_canvas_items(
        &self,
        grid: &HashMap<Cords, Ship>,
        player: &Player,
        image_handler: &GameImageHandler,
    ) -> Vec<CanvasItem> {
        let mut items = Vec::new();

        items.append(&mut self.get_grid_items(
            grid,
            player,
            image_handler
        ));

        items.append(&mut self.create_player_canvas_item(
            player,
            image_handler.player
        ));

        items.append(&mut self.create_player_lives_canvas_items(
            player,
            image_handler.player
        ));

        items
    }

    fn get_grid_items(
        &self,
        grid: &HashMap<Cords, Ship>,
        player: &Player,
        image_handler: &GameImageHandler,
    ) -> Vec<CanvasItem> {
        let mut items = Vec::new();
        let player_position = player.current_position;

        for (cords, ship) in grid {
            if player_position.is_some() && player_position.unwrap() == *cords {
                continue;
            }

            let position = self.calculate_screen_position(cords);

            let image_key = self.select_image_for_ship(
                ship,
                image_handler.fly,
                image_handler.explosion,
                image_handler.bullet_downward,
                image_handler.bullet_upward
            );

            items.push(self.create_canvas_image_item(position, image_key));
        }

        items
    }

    fn create_canvas_image_item(
        &self,
        position: (u32, u32),
        image_key: ImageKey,
    ) -> CanvasItem {
        CanvasItem::Image(
            Area((position.0, position.1), None),
            Shape::Rectangle(0, CELL_SIZE),
            image_key,
        )
    }

    fn calculate_screen_position(
        &self,
        cords: &Cords,
    ) -> (u32, u32) {
        let x = START_X + cords.1 as u32 * (CELL_SIZE.0 + MARGIN);
        let y = START_Y + cords.0 as u32 * (CELL_SIZE.1 + MARGIN);
        (x, y)
    }

    fn select_image_for_ship(
        &self,
        ship: &Ship,
        image_fly: ImageKey,
        explosion: ImageKey,
        bullet_downward: ImageKey,
        bullet_upward: ImageKey,
    ) -> ImageKey {
        match ship.display_type() {
            "fly" => image_fly,
            "explosion" => explosion,
            "bullet" => self.select_bullet_image(ship, bullet_downward, bullet_upward),
            _ => image_fly,
        }
    }

    fn select_bullet_image(
        &self,
        ship: &Ship,
        bullet_downward: ImageKey,
        bullet_upward: ImageKey,
    ) -> ImageKey {
        match ship {
            Ship::Bullet(ai, _) => match ai.actions.first() {
                Some(AIAction::RelativeMove(rel_cords)) if rel_cords.0 > 0 => bullet_downward,
                Some(AIAction::RelativeMove(_)) => bullet_upward,
                _ => bullet_downward,
            },
            _ => bullet_downward,
        }
    }

    fn create_player_canvas_item(
        &self,
        player: &Player,
        player_image: ImageKey,
    ) -> Vec<CanvasItem> {
        let mut items = Vec::new();

        if let Some(pos) = player.current_position {
            let position = self.calculate_screen_position(&pos);
            items.push(self.create_canvas_image_item(position, player_image));
        }

        items
    }

    fn create_player_lives_canvas_items(
        &self,
        player: &Player,
        player_image: ImageKey,
    ) -> Vec<CanvasItem> {
        let mut items = Vec::new();

        let lives_y = START_Y + ROWS as u32 * (CELL_SIZE.1 + MARGIN);

        for live_idx in 0..player.lives {
            let x = START_X + (live_idx as u32) * (CELL_SIZE.0 + MARGIN);
            items.push(self.create_canvas_image_item((x, lives_y), player_image));
        }

        items
    }
}