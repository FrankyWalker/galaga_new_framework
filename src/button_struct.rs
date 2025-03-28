
use rust_on_rails::canvas::{Area, CanvasItem, Shape, Text};
use rust_on_rails::prelude::Context;

pub struct Button {
    pub onclick: Box<dyn FnMut()>,
    pub size: (u32, u32),
    pub offset: (u32, u32),
    pub text: &'static str
}

impl Button {
    pub fn new(onclick: impl FnMut() + 'static, size: (u32, u32), offset: (u32, u32), text: &'static str) -> Button {
        Button {
            onclick: Box::new(onclick),
            size,
            offset,
            text,
        }
    }

    pub fn is_within_bounds(&mut self, x: u32, y: u32) -> bool {
        if x >= self.offset.0 && x <= self.offset.0 + self.size.0 && y >= self.offset.1 && y <= self.offset.1 + self.size.1 {
            (self.onclick)();
            true
        } else {
            false
        }
    }

    pub fn return_canvas_item(&self, ctx: &mut Context) {
        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

        let text_struct = Text::new(self.text, "FFFFFF", 255, Some(800), 15, 38, font.clone());

        let text_size = ctx.messure_text(&text_struct);

        let width = if (text_size.0 + 24) < 48 { 48 } else { text_size.0 + 24 };

        let text_x = self.offset.0 + (width - text_size.0) / 2;
        let text_y = self.offset.1 + (self.size.1 - text_size.1) / 2;

        ctx.draw(
            CanvasItem::Shape(
                Area(self.offset, None),
                Shape::RoundedRectangle(0, (width, 48), 5),
                "FF4500",
                255,
            ));

        ctx.draw(
            CanvasItem::Text(
                Area((text_x, text_y), None),
                text_struct
            ));
    }
}
