use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};
use assets::Assets;
use std::time::Duration;

pub struct Fps {
    pub text: graphics::Text,
    pub cooldown: f64,
}

impl Fps {
    pub const REFRESH_RATE: f64 = 1.0;

    pub fn update(&mut self, ctx: &mut Context, assets: &Assets, dt: Duration) -> GameResult<()> {
        self.cooldown -= timer::duration_to_f64(dt);
        if self.cooldown < 0. {
            self.cooldown = Self::REFRESH_RATE;
            let t = timer::get_fps(ctx);
            self.text = graphics::Text::new(ctx, &format!("FPS: {}", t as u32), &assets.font_small)?;
        }
        Ok(())
    }
}