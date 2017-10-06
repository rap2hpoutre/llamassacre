use Vector2;
use graphics;
use std::time::Duration;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng, random};
use ggez::timer;
use player::Player;

pub enum BonusType {
    GiveOnePoint,
    GiveFivePoint,
    Size2,
    Velocity2,
    Freeze,
}

pub struct Bonus {
    pub tag: BonusType,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub cbox_size: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub image: graphics::Image,
    pub has_collision: bool,
}

impl Bonus {
    fn random(ctx: &mut Context) -> GameResult<Self> {
        let tag = Self::random_type();
        let image = Self::image_by_tag(ctx, &tag)?;
        let b = Bonus {
            tag: tag,
            position: Self::random_position(),
            size: Vector2::new(0.1, 0.1),
            cbox_size: Vector2::new(0.055, 0.075),
            velocity: Self::random_velocity(),
            image: image,
            has_collision: false,
        };
        Ok(b)
    }

    fn random_type() -> BonusType {
        match thread_rng().gen_range(0, 5) {
            0 => BonusType::GiveOnePoint,
            1 => BonusType::GiveFivePoint,
            2 => BonusType::Size2,
            3 => BonusType::Velocity2,
            4 => BonusType::Freeze,
            _ => unreachable!(),
        }
    }

    fn random_position() -> Vector2<f64> {
        Vector2::new((random::<f64>() - 0.5) / 2., 0.5)
    }

    fn random_velocity() -> Vector2<f64> {
        Vector2::new((random::<f64>() - 0.5) / 100., 0.)
    }

    fn image_by_tag(ctx: &mut Context, b: &BonusType) -> GameResult<graphics::Image> {
        Ok(match b {
               &BonusType::GiveOnePoint => {
                   graphics::Image::solid(ctx, 32, graphics::Color::new(0., 0., 1., 1.))?
               }
               &BonusType::GiveFivePoint => {
                   graphics::Image::solid(ctx, 32, graphics::Color::new(0., 1., 0., 1.))?
               }
               &BonusType::Size2 => {
                   graphics::Image::solid(ctx, 32, graphics::Color::new(1., 1., 0., 1.))?
               }
               &BonusType::Velocity2 => {
                   graphics::Image::solid(ctx, 32, graphics::Color::new(1., 0., 0., 1.))?
               }
               &BonusType::Freeze => {
                   graphics::Image::solid(ctx, 32, graphics::Color::new(0.5, 0.7, 1., 1.))?
               }
           })
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
            // This need to be fixed
            BonusType::Size2 => {
                None
                /*
                Some(Mutation {
                         duration: 5.,
                         size_factor: 2.,
                         velocity_factor: Vector2::new(1., 1.),
                         active: true,
                     })
                     */
            }
            BonusType::Velocity2 => {
                Some(Mutation {
                         duration: 5.,
                         size_factor: 1.,
                         velocity_factor: Vector2::new(2., 1.5),
                         active: true,
                     })
            }
            BonusType::Freeze => {
                Some(Mutation {
                         duration: 1.,
                         size_factor: 1.,
                         velocity_factor: Vector2::new(0., 0.),
                         active: true,
                     })
            }
        }
    }
}

pub struct Factory {
    pub cooldown: f64,
}

impl Factory {
    pub fn new() -> Self {
        Self { cooldown: 5. }
    }

    pub fn spawn(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<Option<Bonus>> {
        self.cooldown -= timer::duration_to_f64(dt);
        if self.cooldown < 0. {
            self.cooldown = Self::cooldown();
            println!("spawn");
            return Ok(Some(Bonus::random(ctx)?));
        }
        Ok(None)
    }

    fn cooldown() -> f64 {
        thread_rng().gen_range(1., 2.)
    }
}

#[derive(Debug)]
pub struct Mutation {
    pub duration: f64,
    pub size_factor: f64,
    pub velocity_factor: Vector2<f64>,
    pub active: bool,
}