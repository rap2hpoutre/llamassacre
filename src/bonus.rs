use Vector2;
use graphics;
use std::time::Duration;
use ggez::{Context, GameResult};
use rand::{random, thread_rng, Rng};
use ggez::timer;
use helpers;
use display::Screen;
use player::Player;

pub enum BonusType {
    GiveOnePoint,
    GiveFivePoint,
    Velocity2,
    Freeze,
}

pub struct Bonus {
    pub tag: BonusType,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub cbox_size: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub rotation: f32,
    pub image: graphics::Image,
    pub has_collision: bool,
    pub description: String,
}

impl Bonus {
    fn random(ctx: &mut Context, position: Option<Vector2<f64>>) -> GameResult<Self> {
        let tag = Self::random_type();
        let image = Self::image_by_tag(ctx, &tag)?;
        let text = Self::text_by_tag(&tag);
        let b = Bonus {
            tag: tag,
            position: position.unwrap_or(Self::random_position()),
            size: Vector2::new(0.05, 0.05),
            cbox_size: Vector2::new(0.045, 0.065),
            velocity: Self::random_velocity(),
            image: image,
            rotation: 0.,
            has_collision: false,
            description: text,
        };
        Ok(b)
    }

    fn random_type() -> BonusType {
        match thread_rng().gen_range(0, 4) {
            0 => BonusType::GiveOnePoint,
            1 => BonusType::GiveFivePoint,
            2 => BonusType::Velocity2,
            3 => BonusType::Freeze,
            _ => unreachable!(),
        }
    }

    fn random_position() -> Vector2<f64> {
        Vector2::new((random::<f64>() - 0.5) / 2., 0.5)
    }

    fn random_velocity() -> Vector2<f64> {
        Vector2::new((random::<f64>() - 0.5) / 50., 0.01)
    }

    fn image_by_tag(ctx: &mut Context, b: &BonusType) -> GameResult<graphics::Image> {
        Ok(match b {
            &BonusType::GiveOnePoint => {
                let mut a = graphics::Image::new(ctx, "/bonus_1.png")?;
                a.set_filter(graphics::FilterMode::Nearest);
                a
            }
            &BonusType::GiveFivePoint => {
                let mut a = graphics::Image::new(ctx, "/bonus_0.png")?;
                a.set_filter(graphics::FilterMode::Nearest);
                a
            }

            &BonusType::Velocity2 => {
                let mut a = graphics::Image::new(ctx, "/bonus_2.png")?;
                a.set_filter(graphics::FilterMode::Nearest);
                a
            }
            &BonusType::Freeze => {
                let mut a = graphics::Image::new(ctx, "/bonus_3.png")?;
                a.set_filter(graphics::FilterMode::Nearest);
                a
            }
        })
    }


    fn text_by_tag(b: &BonusType) -> String {
        match b {
            &BonusType::GiveOnePoint => "score +1",
            &BonusType::GiveFivePoint => "score +5",
            &BonusType::Velocity2 => "speed x2",
            &BonusType::Freeze => "freeze",
        }.to_string()
    }

    pub fn draw(&mut self, ctx: &mut Context, screen: &Screen) -> GameResult<()> {
        let size = self.size;
        let position = self.position;
        let image = &self.image;
        let dest = helpers::point_from_position(position, screen);
        let scale = helpers::scale(size, screen, image);
        let draw_param = graphics::DrawParam {
            dest: dest,
            scale: scale,
            rotation: self.rotation,
            ..Default::default()
        };
        graphics::draw_ex(ctx, image, draw_param)?;
        Ok(())
    }

    pub fn apply(&self, p: &mut Player) -> Option<Mutation> {
        match self.tag {
            BonusType::GiveOnePoint => {
                p.score += 1;
                None
            }
            BonusType::GiveFivePoint => {
                p.score += 5;
                None
            }
            BonusType::Velocity2 => Some(Mutation {
                duration: 7.5,
                size_factor: 1.,
                velocity_factor: Vector2::new(2., 1.5),
                active: true,
            }),
            BonusType::Freeze => Some(Mutation {
                duration: 2.,
                size_factor: 1.,
                velocity_factor: Vector2::new(0., 0.),
                active: true,
            }),
        }
    }
}

pub struct BonusText {
    pub text: graphics::Text,
    pub position: Vector2<f64>,
    pub cooldown: f64,
}

pub struct Factory {
    pub cooldown: f64,
    pub image: graphics::Image,
    pub alt_image: graphics::Image,
    pub alt_image_cooldown: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub size: Vector2<f64>,
    pub rotation: f32,
    pub rotation_velocity: f32,
}

impl Factory {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = graphics::Image::new(ctx, "/divin2.png")?;
        let alt_image = graphics::Image::new(ctx, "/divin1.png")?;
        Ok(Self {
            cooldown: 20.,
            image: image,
            alt_image_cooldown: 0.,
            alt_image: alt_image,
            position: Vector2::new(0., 1.1),
            size: Vector2::new(0.1, 0.1),
            velocity: Vector2::new(0.15, 0.0),
            rotation: 0.,
            rotation_velocity: -1.,
        })
    }

    pub fn spawn(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<Option<Bonus>> {
        self.cooldown -= timer::duration_to_f64(dt);
        self.alt_image_cooldown -= timer::duration_to_f64(dt);
        if self.cooldown < 0. && self.position.x < 0.4 && self.position.x > -0.4 {
            self.alt_image_cooldown = 1.;
            self.cooldown = Self::cooldown();
            return Ok(Some(Bonus::random(ctx, Some(self.position))?));
        }
        Ok(None)
    }

    pub fn draw(&mut self, ctx: &mut Context, screen: &Screen) -> GameResult<()> {
        let size = screen.size_to_pixel(Vector2::new(0.1625, 0.34));
        let position = self.position;
        let image = if self.alt_image_cooldown <= 0. {
            &self.image
        } else {
            &self.alt_image
        };
        
        let dest = helpers::point_from_position(position, screen);
        let draw_param = graphics::DrawParam {
            dest: dest,
            scale: graphics::Point {
                x: size.x as f32 / image.width() as f32,
                y: size.y as f32 / image.height() as f32,
            },
            rotation: self.rotation,
            ..Default::default()
        };
        graphics::draw_ex(ctx, image, draw_param)?;
        Ok(())
    }

    fn cooldown() -> f64 {
        thread_rng().gen_range(0., 30.)
    }

    pub fn update(&mut self, seconds: f64) {
        if self.position.y > 0.3 {
            self.position.y -= seconds / 50.;
        } else {
            self.position.y = 0.3;
            self.position += self.velocity * seconds;
            if self.position.x > 1.7 || self.position.x < -1.7 {
                self.velocity.x *= thread_rng().gen_range(-1.2, -0.7);
                self.rotation = 0.;
                match thread_rng().gen_range(0, 3) {
                    0 => {                        
                        self.rotation_velocity = 0.;
                    },
                    _ => {
                        self.rotation_velocity = thread_rng().gen_range(-1.5, -0.5);
                    },
                };
            }
        }
        self.rotation += seconds as f32 * self.rotation_velocity;
        self.rotation_velocity -= self.rotation / 30.;
    }
}

#[derive(Debug)]
pub struct Mutation {
    pub duration: f64,
    pub size_factor: f64,
    pub velocity_factor: Vector2<f64>,
    pub active: bool,
}
