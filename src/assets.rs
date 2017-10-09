use ggez::graphics;
use ggez::audio;
use ggez::{Context, GameResult};

// Assets
pub struct Assets {
    pub font: graphics::Font,
    pub blood: graphics::Image,
    pub bg: graphics::Image,
    pub title: graphics::Text,
    pub authors: graphics::Text,
    pub instructions_p1: Vec<graphics::Text>,
    pub instructions_p2: Vec<graphics::Text>,
    pub jump: audio::Source,
    pub death: audio::Source,
    pub take_bonus: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 18)?;
        let font_small = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 12)?;
        let blood = graphics::Image::new(ctx, "/blood.png")?;
        let title = graphics::Text::new(ctx, "AYY LLAMA", &font)?;
        let authors = graphics::Text::new(ctx, "by rap2h & iorekb", &font)?;
        let instructions_p1 = vec![
            graphics::Text::new(ctx, "Player 1:", &font_small)?,
            graphics::Text::new(ctx, "E: jump", &font_small)?,
            graphics::Text::new(ctx, "S: left", &font_small)?,
            graphics::Text::new(ctx, "F: right", &font_small)?,
        ];
        let instructions_p2 = vec![
            graphics::Text::new(ctx, "Player 2:", &font_small)?,
            graphics::Text::new(ctx, "up: jump", &font_small)?,
            graphics::Text::new(ctx, "left: left", &font_small)?,
            graphics::Text::new(ctx, "right: right", &font_small)?,
        ];
        let jump = audio::Source::new(ctx, "/jump.wav")?;
        let take_bonus = audio::Source::new(ctx, "/jump.wav")?;
        let death = audio::Source::new(ctx, "/death.ogg")?;
        let mut bg = graphics::Image::new(ctx, "/bg3.png")?;
        bg.set_filter(graphics::FilterMode::Nearest);
        let s = Assets {
            font: font,
            blood: blood,
            bg: bg,
            title: title,
            jump: jump,
            death: death,
            take_bonus: take_bonus,
            authors: authors,
            instructions_p1: instructions_p1,
            instructions_p2: instructions_p2,
        };
        Ok(s)
    }
}