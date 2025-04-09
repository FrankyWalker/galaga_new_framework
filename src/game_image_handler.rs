use rust_on_rails::prelude::*;

#[derive(Clone)]
pub struct GameImageHandler {
    pub fly: ImageKey,
    pub explosion: ImageKey,
    pub bullet_downward: ImageKey,
    pub bullet_upward: ImageKey,
    pub player: ImageKey,
}
//this inserts images into the app so it can be accessed from around the app
impl GameImageHandler {
    pub fn new(ctx: &mut Context) -> Self {
        let fly = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let explosion = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());
        let bullet_downward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_downward.png")).unwrap().into());
        let bullet_upward = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/bullet_upward.png")).unwrap().into());
        let player = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/spaceship.png")).unwrap().into());

        Self {
            fly,
            explosion,
            bullet_downward,
            bullet_upward,
            player,
        }
    }
}