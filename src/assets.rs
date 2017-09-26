use ggez::graphics;
use ggez::audio;
use ggez::{Context, GameResult};

// Assets
pub struct Assets {
    pub font: graphics::Font,
    pub blood: graphics::Image,
    pub title: graphics::Text,
    pub jump: audio::Source,
    pub death: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 12)?;
        let blood = graphics::Image::solid(ctx, 22, graphics::Color::new(1., 0., 0., 1.))?;
        let title = graphics::Text::new(ctx, "ayy llama", &font)?;
        let jump = audio::Source::new(ctx, "/jump.wav")?;
        let death = audio::Source::new(ctx, "/death.ogg")?;
        let s = Assets {
            font: font,
            blood: blood,
            title: title,
            jump: jump,
            death: death,
        };
        Ok(s)
    }
}