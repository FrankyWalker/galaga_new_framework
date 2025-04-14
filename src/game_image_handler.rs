use rust_on_rails::prelude::*;

#[derive(Clone)]
pub struct GameImageHandler {
    pub fly: ImageKey,
    pub tiki_fly: ImageKey,
    pub northrop_fly: ImageKey,
    pub b2_fly: ImageKey,
    pub explosion: ImageKey,
    pub bullet_downward: ImageKey,
    pub bullet_upward: ImageKey,
    pub player: ImageKey,
}

impl GameImageHandler {
    pub fn new(ctx: &mut Context) -> Self {
        let fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let explosion = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());
        let bullet_downward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_downward.png")).unwrap().into());
        let bullet_upward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_upward.png")).unwrap().into());
        let player = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/spaceship.png")).unwrap().into());

        let tiki_fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/tiki_fly.png")).unwrap().into());
        let northrop_fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/northrop.png")).unwrap().into());
        let b2_fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/b-2.png")).unwrap().into());

        Self {
            fly,
            tiki_fly,
            northrop_fly,
            b2_fly,
            explosion,
            bullet_downward,
            bullet_upward,
            player,
        }
    }
}
