use rand::Rng;
use rust_on_rails::prelude::*;

#[derive(Clone)]
pub struct Star {
    pub x: u32,
    pub y: f32,
    pub speed: f32,
    pub color: &'static str,
    pub size: (u32, u32),
}

//This is the star background a animation of stars moving downward and wrapping back to the top
pub struct StarBackground {
    stars: Vec<Star>,
    window_size: (u32, u32),
}

impl StarBackground {
    pub fn new(window_size: (u32, u32)) -> Self {
        let mut stars = Vec::new();
        let mut rng = rand::rng();

        for _ in 0..150 {
            let color = match rng.random_range(0..4) {
                0 => "0099FF",
                1 => "FF6600",
                2 => "FF0000",
                _ => "FFFF00",
            };

            stars.push(Star {
                x: rng.random_range(0..window_size.0),
                y: rng.random_range(0.0..window_size.1 as f32),
                speed: rng.random_range(0.5..3.0),
                color,
                size: (rng.random_range(1..4), rng.random_range(1..4)),
            });
        }

        StarBackground {
            stars,
            window_size,
        }
    }

    pub fn on_tick(&mut self) {
        let mut rng = rand::rng();
        for star in &mut self.stars {
            star.y += star.speed;

            if star.y > self.window_size.1 as f32 {
                star.y = 0.0;
                star.x = rng.random_range(0..self.window_size.0);
                star.speed = rng.random_range(0.5..3.0);

                if rng.random_bool(0.3) {
                    star.color = match rng.random_range(0..4) {
                        0 => "0099FF",
                        1 => "FF6600",
                        2 => "FF0000",
                        _ => "FFFF00",
                    };
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for star in &self.stars {
            ctx.draw(CanvasItem::Shape(
                Area((star.x, star.y as u32), None),
                Shape::Rectangle(0, star.size),
                star.color,
                255,
            ));
        }
    }
}