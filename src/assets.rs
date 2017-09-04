use ggez::graphics;
use ggez::{Context, GameResult};

// Assets
pub struct Assets {
    pub font: graphics::Font,
    pub blood: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;
        let blood = graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 0., 1.))?;
        let s = Assets { font: font, blood: blood };
        Ok(s)
    }
}