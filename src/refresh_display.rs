use rust_on_rails::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::ship::{AIAction, Ship, ShipAI};

use crate::structs::{Cords, ShipAction, COLUMNS, ROWS};
use crate::player::{Player, integrate_player_to_game};
use crate::settings::{GameSettings, create_game_settings, spawn_initial_flies};

use crate::Star;


pub(crate) fn refresh_display(
    grid: &mut HashMap<Cords, Ship>,
    items: &mut Vec<CanvasItem>,
    player: &mut Player,
    settings: &GameSettings,
    rows: u32,
    cols: u32,
    cell_size: (u32, u32),
    margin: u32,
    window_size: (u32, u32),
    font: FontKey,
    stars: &Vec<Star>,
    image_fly: ImageKey,
    explosion: ImageKey,
    bullet_downward: ImageKey,
    bullet_upward: ImageKey,
    player_image: ImageKey,
) {
    items.clear();

    let grid_width = cols * (cell_size.0 + margin) - margin;
    let grid_height = rows * (cell_size.1 + margin) - margin;

    let start_x = 5;
    let start_y = 50;

    // Draw grid background
    items.push(CanvasItem::Shape(
        Area((start_x - margin, start_y - margin), None),
        Shape::Rectangle(0, (grid_width + 2 * margin, grid_height + 2 * margin)),
        "000000",
        200,
    ));

    // Draw grid cells
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col * (cell_size.0 + margin);
            let y = start_y + row * (cell_size.1 + margin);

            items.push(CanvasItem::Shape(
                Area((x - 1, y - 1), None),
                Shape::RoundedRectangle(0, (cell_size.0 + 2, cell_size.1 + 2), 10),
                "000000",
                255,
            ));

            items.push(CanvasItem::Shape(
                Area((x, y), None),
                Shape::RoundedRectangle(0, cell_size, 10),
                "000000",
                100,
            ));
        }
    }

    // Draw ships on grid
    for (cords, ship) in grid.iter() {
        if player.current_position.is_some() && player.current_position.unwrap() == *cords {
            continue;
        }

        let x = start_x + cords.1 as u32 * (cell_size.0 + margin);
        let y = start_y + cords.0 as u32 * (cell_size.1 + margin);

        let image = match ship.display_type() {
            "fly" => image_fly,
            "explosion" => explosion,
            "bullet" => {
                match ship {
                    Ship::Bullet(ai, _, _) => {
                        if let Some(AIAction::RelativeMove(rel_cords)) = ai.actions.get(0) {
                            if rel_cords.0 > 0 {
                                bullet_downward
                            } else {
                                bullet_upward
                            }
                        } else {
                            bullet_downward
                        }
                    },
                    _ => bullet_downward,
                }
            },
            _ => image_fly,
        };

        items.push(CanvasItem::Image(
            Area((x, y), None),
            Shape::Rectangle(0, cell_size),
            image,
        ));
    }

    // Draw player on grid
    if let Some(pos) = player.current_position {
        let x = start_x + pos.1 as u32 * (cell_size.0 + margin);
        let y = start_y + pos.0 as u32 * (cell_size.1 + margin);

        items.push(CanvasItem::Image(
            Area((x, y), None),
            Shape::Rectangle(0, cell_size),
            player_image,
        ));
    }

    // Draw player lives
    for live_idx in 0..player.lives {
        let live_cords = (rows, live_idx as u8);
        let x = start_x + live_cords.1 as u32 * (cell_size.0 + margin);
        let y = start_y + live_cords.0 as u32 * (cell_size.1 + margin);

        items.push(CanvasItem::Image(
            Area((x, y), None),
            Shape::Rectangle(0, cell_size),
            player_image,
        ));
    }
}
