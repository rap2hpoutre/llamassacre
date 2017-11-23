use cgmath::Vector2;
use controls::Controls;
use animation::PlayerAnimation;
use bonus::Mutation;
use assets::Assets;
use ggez::{Context, GameResult};
use helpers;
use display::Screen;
use ggez::graphics;
use timer;
use std::time::Duration;

// Players
#[derive(Debug)]
pub enum Facing {
    Left,
    Right,
}

// Players
#[derive(Debug)]
pub enum PlayerType {
    Player1,
    Player2,
}

#[derive(Debug)]
pub struct Player {
    pub tag: PlayerType,
    pub position: Vector2<f64>,
    pub facing: Facing,
    pub size: Vector2<f64>,
    pub cbox_size: Vector2<f64>,
    pub max_velocity: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub input_axis: Vector2<f64>,
    pub controls: Controls,
    pub score: u32,
    pub animation: PlayerAnimation,
    pub mutations: Vec<Mutation>,
}

impl Player {
    pub fn new(controls: Controls, tag: PlayerType, animation: PlayerAnimation) -> Player {
        let position = match tag {
            PlayerType::Player1 => Vector2::new(0.25, ::GROUND_Y),
            PlayerType::Player2 => Vector2::new(-0.25, ::GROUND_Y),
        };
        let facing = match tag {
            PlayerType::Player1 => Facing::Left,
            PlayerType::Player2 => Facing::Right,
        };
        Player {
            tag: tag,
            position: position,
            facing: facing,
            size: Vector2::new(0.1, 0.1),
            cbox_size: Vector2::new(0.055, 0.075),
            max_velocity: Vector2::new(0.2, 1.0),
            velocity: Vector2::new(0., 0.),
            input_axis: Vector2::new(0., 0.),
            controls: controls,
            score: 0,
            animation: animation,
            mutations: vec![],
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, screen: &Screen) -> GameResult<()> {
        let mut size = self.size;
        let mut position = self.position;
        for m in &mut self.mutations {
            size *= m.size_factor;
        }
        let initial_max_velocity = self.max_velocity;
        let max_velocity = self.max_velocity_mutated();
        position.y += (size.y - self.size.y) / 1.5;
        let dest = helpers::point_from_position(position, screen);
        let player_image = helpers::player_image(self);
        let scale = helpers::scale(size, screen, player_image);
        let draw_param = graphics::DrawParam {
            dest: dest,
            scale: scale,
            ..Default::default()
        };
        if max_velocity.x == 0. {
            graphics::set_color(ctx, (200, 200, 255).into())?;
        } else if max_velocity.x > initial_max_velocity.x {
            graphics::set_color(ctx, (255, 200, 200).into())?;
        }
        graphics::draw_ex(ctx, player_image, draw_param)?;

        if (max_velocity.x == 0.) || (max_velocity.x > initial_max_velocity.x) {
            graphics::set_color(ctx, (255, 255, 255).into())?;
        }

        Ok(())
    }

    pub fn update_mutations(&mut self, dt: Duration) {
        self.mutations.retain(|m| m.duration > 0.);
        for m in &mut self.mutations {
            m.duration -= timer::duration_to_f64(dt);
        }
    }

    fn max_velocity_mutated(&self) -> Vector2<f64> {
        let mut max_velocity = self.max_velocity; // Maybe I need to copy
        for m in &self.mutations {
            max_velocity.x *= m.velocity_factor.x;
            max_velocity.y *= m.velocity_factor.y;
        }
        max_velocity
    }

    pub fn update_position(
        &mut self,
        screen: &Screen,
        seconds: f64,
        assets: &Assets,
    ) -> GameResult<()> {
        let max_velocity = self.max_velocity_mutated();

        self.velocity.x = seconds * max_velocity.x * self.input_axis.x;

        if helpers::is_on_top(self.position, Vector2::new(0., ::GROUND_Y), screen) {
            if max_velocity.y == 0. {
                if self.velocity.y > 0. {
                    self.velocity.y = 0.;
                }
                self.velocity.y -= seconds * self.max_velocity.y / ::GRAVITY_MAGIC_NUMBER;
            } else {
                self.velocity.y -= seconds * max_velocity.y / ::GRAVITY_MAGIC_NUMBER;
            }
        } else {
            self.velocity.y = seconds * max_velocity.y * self.input_axis.y;
            if self.input_axis.y != 0.0 && max_velocity.y > 0. {
                match self.tag {
                    PlayerType::Player1 => assets.jump[0].play()?,
                    PlayerType::Player2 => assets.jump[1].play()?,
                };
;
            }
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        if self.position.y < ::GROUND_Y {
            self.position.y = ::GROUND_Y;
        }

        if self.position.x > 0.5 {
            self.position.x = -0.5;
        } else if self.position.x < -0.5 {
            self.position.x = 0.5;
        }

        Ok(())
    }
}
