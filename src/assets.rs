use ggez::graphics;
use ggez::audio;
use ggez::{Context, GameResult};

// Assets
pub struct Assets {
    pub font: graphics::Font,
    pub blood: graphics::Image,
    pub bg: graphics::Image,
    pub title: graphics::Text,
    pub jump: audio::Source,
    pub death: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 12)?;
        let blood = graphics::Image::new(ctx, "/blood.png")?;
        let title = graphics::Text::new(ctx, "ayy llama", &font)?;
        let jump = audio::Source::new(ctx, "/jump.wav")?;
        let death = audio::Source::new(ctx, "/death.ogg")?;
        let mut bg = graphics::Image::new(ctx, "/bg.png")?;
        bg.set_filter(graphics::FilterMode::Nearest);
        let s = Assets {
            font: font,
            blood: blood,
            bg: bg,
            title: title,
            jump: jump,
            death: death,
        };
        Ok(s)
    }
}