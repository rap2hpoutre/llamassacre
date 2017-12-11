use ggez::graphics;
use ggez::audio;
use ggez::{Context, GameResult};

// Assets
pub struct Assets {
    pub font: graphics::Font,
    pub font_small: graphics::Font,
    pub blood: graphics::Image,
    pub bg: graphics::Image,
    pub shadow: graphics::Image,
    pub title: graphics::Text,
    pub authors: graphics::Text,
    pub single: graphics::Text,
    pub instructions_p1: Vec<graphics::Text>,
    pub instructions_p2: Vec<graphics::Text>,
    pub credits: Vec<graphics::Text>,
    pub jump: Vec<audio::Source>,
    pub death: audio::Source,
    pub take_bonus: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 18)?;
        let font_small = graphics::Font::new(ctx, "/TravelingTypewriter.ttf", 12)?;
        let blood = graphics::Image::new(ctx, "/blood.png")?;
        let title = graphics::Text::new(ctx, "LLAMASSACRE", &font)?;
        let authors = graphics::Text::new(ctx, "press SPACE to start, C for credits", &font_small)?;
        let single = graphics::Text::new(ctx, "single player? press ESC to quit", &font_small)?;
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
        let credits = vec![
            graphics::Text::new(ctx, "Artwork & design: iorekb", &font_small)?,
            graphics::Text::new(ctx, "Prog, artwork & design: rap2h", &font_small)?,
            graphics::Text::new(ctx, "Game engine: ggez", &font_small)?,
            graphics::Text::new(ctx, "Font: TravelingTypewriter", &font_small)?,
            graphics::Text::new(ctx, "Sounds plundered on freesound.org", &font_small)?,
            graphics::Text::new(ctx, "Artwork highly inspired by various artists", &font_small)?,
        ];
        let jump = audio::Source::new(ctx, "/jump.ogg")?;
        let jump2 = audio::Source::new(ctx, "/jump.ogg")?;
        let take_bonus = audio::Source::new(ctx, "/bonus.wav")?;
        let death = audio::Source::new(ctx, "/death.ogg")?;
        let mut bg = graphics::Image::new(ctx, "/bg10.png")?;
        let shadow = graphics::Image::new(ctx, "/shadow2.png")?;
        bg.set_filter(graphics::FilterMode::Nearest);
        let s = Assets {
            font: font,
            font_small: font_small,
            blood: blood,
            bg: bg,
            title: title,
            jump: vec![jump, jump2],
            death: death,
            take_bonus: take_bonus,
            authors: authors,
            instructions_p1: instructions_p1,
            instructions_p2: instructions_p2,
            shadow: shadow,
            single: single,
            credits: credits
        };
        Ok(s)
    }
}