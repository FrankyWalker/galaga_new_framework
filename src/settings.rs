use std::time::Duration;
use rust_on_rails::prelude::*;
use rust_on_rails::canvas::{Area, CanvasItem, Shape, Text};

#[derive(Clone)]
pub struct Settings {
    pub value_stats: Values,
    settings_buttons: Buttons,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            value_stats: Values::new(),
            settings_buttons: Buttons::new(),
        }
    }

    pub fn draw(&self, ctx: &mut Context, font: FontKey) {
        self.value_stats.draw(ctx, font);
        self.settings_buttons.draw(ctx, font);
    }

    pub fn handle_click(&mut self, x: u32, y: u32) -> Option<ButtonAction> {
        if let Some(action) = self.settings_buttons.find_clicked_button(x, y) {
            self.value_stats.handle_action(action)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Values {
    pub invincible: bool,
    pub fly_move: bool,
    pub laser_shoot: bool,
    pub number_of_flies: u32,
    pub fly_speed: Duration,
    pub laser_speed: Duration,
    pub shooting_randomness: u32,
}

impl Values {
    pub fn new() -> Self {
        Self {
            invincible: false,
            fly_move: true,
            laser_shoot: true,
            number_of_flies: 17,
            fly_speed: Duration::from_millis(900),
            laser_speed: Duration::from_millis(300),
            shooting_randomness: 10,
        }
    }

    fn handle_action(&mut self, action: ButtonAction) -> Option<ButtonAction> {
        match action {
            ButtonAction::FlySpeedDecrease => {
                let millis = self.fly_speed.as_millis().saturating_sub(10);
                self.fly_speed = Duration::from_millis(if millis == 0 { 1 } else { millis as u64 });
                println!("Fly Speed reduced to: {} ms", self.fly_speed.as_millis());
            }
            ButtonAction::FlySpeedIncrease => {
                let millis = self.fly_speed.as_millis().saturating_add(10);
                self.fly_speed = Duration::from_millis(millis as u64);
                println!("Fly Speed increased to: {} ms", self.fly_speed.as_millis());
            }
            ButtonAction::LaserSpeedIncrease => {
                let millis = self.laser_speed.as_millis().saturating_add(10);
                self.laser_speed = Duration::from_millis(if millis > 500 { 500 } else { millis as u64 });
                println!("Laser Speed increased to: {} ms", self.laser_speed.as_millis());
            }
            ButtonAction::LaserSpeedDecrease => {
                let millis = self.laser_speed.as_millis().saturating_sub(10);
                self.laser_speed = Duration::from_millis(if millis == 0 { 1 } else { millis as u64 });
                println!("Laser Speed decreased to: {} ms", self.laser_speed.as_millis());
            }
            ButtonAction::FliesDecrease => {
                self.number_of_flies = self.number_of_flies.saturating_sub(1);
            }
            ButtonAction::FliesIncrease => {
                self.number_of_flies = self.number_of_flies.saturating_add(1);
            }
            ButtonAction::ToggleInvincible => {
                self.invincible = !self.invincible;
            }
            ButtonAction::ToggleFlyMovement => {
                self.fly_move = !self.fly_move;
            }
            ButtonAction::ToggleLaserShooting => {
                self.laser_shoot = !self.laser_shoot;
            }
            ButtonAction::Reset => {
                return Some(ButtonAction::Reset);
            }
        }

        Some(action)
    }

    fn draw(&self, ctx: &mut Context, font: FontKey) {
        self.draw_stats_text(ctx, 30, 780, format!("Fly Speed: {} ms", self.fly_speed.as_millis()), font);
        self.draw_stats_text(ctx, 310, 780, format!("Laser Speed: {} ms", self.laser_speed.as_millis()), font);
        self.draw_stats_text(ctx, 570, 780, format!("Flies: {}", self.number_of_flies), font);
        self.draw_stats_text(ctx, 570, 810, format!("Invincible: {}", if self.invincible { "ON" } else { "OFF" }), font);
        self.draw_stats_text(ctx, 30, 810, format!("Fly Movement: {}", if self.fly_move { "ON" } else { "OFF" }), font);
        self.draw_stats_text(ctx, 310, 810, format!("Laser Shoot: {}", if self.laser_shoot { "ON" } else { "OFF" }), font);
    }

    fn draw_stats_text(&self, ctx: &mut Context, x: u32, y: u32, content: String, font: FontKey) {
        ctx.draw(
            CanvasItem::Text(
                Area((x, y), None),
                Text::new(
                    content.leak(),
                    "FFFFFF",
                    255,
                    Some(800),
                    20,
                    25,
                    font
                )
            )
        );
    }
}

#[derive(Clone)]
pub enum ButtonAction {
    FlySpeedDecrease,
    FlySpeedIncrease,
    LaserSpeedIncrease,
    LaserSpeedDecrease,
    FliesDecrease,
    FliesIncrease,
    ToggleInvincible,
    ToggleFlyMovement,
    ToggleLaserShooting,
    Reset,
}

#[derive(Clone)]
struct Button {
    pub action: ButtonAction,
    pub size: (u32, u32),
    pub offset: (u32, u32),
    pub text: &'static str,
}

impl Button {
    fn new(action: ButtonAction, size: (u32, u32), offset: (u32, u32), text: &'static str) -> Button {
        Button {
            action,
            size,
            offset,
            text,
        }
    }

    fn is_within_bounds(&self, x: u32, y: u32) -> bool {
        x >= self.offset.0 && x <= self.offset.0 + self.size.0 &&
            y >= self.offset.1 && y <= self.offset.1 + self.size.1
    }
}

#[derive(Clone)]
struct Buttons {
    buttons: Vec<Button>,
}

impl Buttons {
    fn new() -> Self {
        Self {
            buttons: vec![
                Button::new(ButtonAction::FlySpeedIncrease, (130, 40), (30, 910), "Fly Speed +"),
                Button::new(ButtonAction::FlySpeedDecrease, (130, 40), (30, 850), "Fly Speed -"),
                Button::new(ButtonAction::LaserSpeedIncrease, (165, 40), (190, 910), "Laser Speed +"),
                Button::new(ButtonAction::LaserSpeedDecrease, (165, 40), (190, 850), "Laser Speed -"),
                Button::new(ButtonAction::FliesDecrease, (73, 40), (380, 910), "Flies -"),
                Button::new(ButtonAction::FliesIncrease, (73, 40), (380, 850), "Flies +"),
                Button::new(ButtonAction::ToggleInvincible, (105, 40), (480, 910), "Invincible"),
                Button::new(ButtonAction::ToggleFlyMovement, (100, 40), (480, 850), "Fly Move"),
                Button::new(ButtonAction::ToggleLaserShooting, (75, 40), (615, 910), "Lasers"),
                Button::new(ButtonAction::Reset, (170, 40), (860, 880), "Save & Restart"),
            ]
        }
    }

    fn draw(&self, ctx: &mut Context, font: FontKey) {
        for button in &self.buttons {
            self.draw_button(ctx, button, font);
        }
    }

    fn draw_button(&self, ctx: &mut Context, button: &Button, font: FontKey) {
        let text_struct = Text::new(button.text, "FFFFFF", 255, Some(800), 25, 38, font);
        let text_size = ctx.messure_text(&text_struct);

        let text_x = match text_size.0 < button.size.0 {
            true => button.offset.0 + (button.size.0 - text_size.0) / 2,
            false => button.offset.0,
        };

        let text_y = match text_size.1 < button.size.1 {
            true => button.offset.1 + (button.size.1 - text_size.1) / 2,
            false => button.offset.1,
        };

        ctx.draw(
            CanvasItem::Shape(
                Area(button.offset, None),
                Shape::RoundedRectangle(0, (button.size.0, 48), 5),
                "FF4500",
                255,
            )
        );

        ctx.draw(
            CanvasItem::Text(
                Area((text_x, text_y), None),
                text_struct
            )
        );
    }

    fn find_clicked_button(&self, x: u32, y: u32) -> Option<ButtonAction> {
        for button in &self.buttons {
            if button.is_within_bounds(x, y) {
                return Some(button.action.clone());
            }
        }
        None
    }
}
