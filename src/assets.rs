use ggez::graphics;
use ggez::{Context, GameResult};
use animation::PlayerAnimation;

// Assets
pub struct Assets {
    pub font: graphics::Font
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;
        let s = Assets { font: font };
        Ok(s)
    }

    //pub fn player_image(&mut self, player: &Player) -> &mut graphics::Image {
    //    player.animation.current()
    //}
}

pub fn player1_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 0., 1.))?,
            graphics::Image::solid(ctx, 22, graphics::Color::new(1., 1., 0., 1.))?,
        ],
        jump: graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 1., 1.))?,
        fall: graphics::Image::solid(ctx, 22, graphics::Color::new(0., 1., 1., 1.))?,
        stand: graphics::Image::solid(ctx, 22, graphics::Color::new(1., 1., 1., 1.))?,      
        time: 0. 
    };
    Ok(r)
}

pub fn player2_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 0., 1.))?,
            graphics::Image::solid(ctx, 22, graphics::Color::new(1., 1., 0., 1.))?,
        ],
        jump: graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 1., 1.))?,
        fall: graphics::Image::solid(ctx, 22, graphics::Color::new(0., 1., 1., 1.))?,
        stand: graphics::Image::solid(ctx, 22, graphics::Color::new(1., 1., 1., 1.))?,      
        time: 0. 
    };
    Ok(r)
}