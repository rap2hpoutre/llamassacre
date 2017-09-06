extern crate ggez;
extern crate cgmath;
extern crate rand;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use std::time::Duration;
use cgmath::Vector2;
use cgmath::MetricSpace;
use controls::Controls;
use display::Screen;
use assets::Assets;
use ui::Fps;
use player::{Player, PlayerType, Facing};
use particles::Blood;
use helpers::*;

mod controls;
mod display;
mod assets;
mod player;
mod animation;
mod particles;
mod helpers;
mod ui;

// Main state
struct MainState {
    screen: Screen,
    assets: Assets,
    players: [Player; 2],
    text_scores: [graphics::Text; 2],
    blood_particles: Vec<Blood>,
    fps: Fps
}

impl MainState {
    const DESIRED_FPS: u64 = 60;
    const GROUND_Y: f64 = 0.;
    const GRAVITY_MAGIC_NUMBER: f64 = 20.;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let text_scores = [graphics::Text::new(ctx, "0", &assets.font)?,
                           graphics::Text::new(ctx, "0", &assets.font)?];
        let fps = Fps {
            text: graphics::Text::new(ctx, "FPS:", &assets.font)?,
            cooldown: 1.0,
        };
        let player1 = Player::new(Controls {
                                      up: event::Keycode::Up,
                                      left: event::Keycode::Left,
                                      right: event::Keycode::Right,
                                  },
                                  PlayerType::Player1,
                                  player1_animation(ctx)?);
        let player2 = Player::new(Controls {
                                      up: event::Keycode::E,
                                      left: event::Keycode::S,
                                      right: event::Keycode::F,
                                  },
                                  PlayerType::Player2,
                                  player2_animation(ctx)?);
        let s = MainState {
            assets: assets,
            text_scores: text_scores,
            screen: Screen::new(),
            players: [player1, player2],
            blood_particles: vec![],
            fps: fps
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        if !timer::check_update_time(ctx, Self::DESIRED_FPS) {
            return Ok(());
        }
        let seconds = 1.0 / (Self::DESIRED_FPS as f64);

        // Update players position
        {
            for player in &mut self.players {
                player.velocity.x = seconds * player.max_velocity.x * player.input_axis.x;

                if self.screen
                       .position_to_pixel(Vector2::new(0., Self::GROUND_Y))
                       .y as u32 >
                   self.screen.position_to_pixel(player.position).y as u32 {
                    player.velocity.y -= seconds * player.max_velocity.y /
                                         Self::GRAVITY_MAGIC_NUMBER;
                } else {
                    player.velocity.y = seconds * player.max_velocity.y * player.input_axis.y;
                }

                player.position.x += player.velocity.x;
                player.position.y += player.velocity.y;

                if player.position.y < Self::GROUND_Y {
                    player.position.y = Self::GROUND_Y;
                }
            }
        }

        // Particles
        {
            for blood_particle in &mut self.blood_particles {
                blood_particle.velocity.y -= seconds * 1.2 / Self::GRAVITY_MAGIC_NUMBER;
                blood_particle.position.x += blood_particle.velocity.x;
                blood_particle.position.y += blood_particle.velocity.y;
            }
            self.blood_particles
                .retain(|blood_particle| blood_particle.position.y > -0.5);
        }

        // Collision
        {
            for i in 0..self.players.len() {
                let n = i + 1;
                for j in n..self.players.len() {
                    let distance = self.players[i]
                        .position
                        .distance(self.players[j].position);
                    if distance < self.players[i].cbox_size.x {
                        let pos_y_i = self.players[i].position.y;
                        let pos_y_j = self.players[j].position.y;
                        let (frag, killer, victim) =
                            if pos_y_i > pos_y_j && self.players[i].velocity.y < 0. {
                                (true, Some(i), Some(j))
                            } else if pos_y_j > pos_y_i && self.players[j].velocity.y < 0. {
                                (true, Some(j), Some(i))
                            } else {
                                (false, None, None)
                            };
                        if frag {
                            let (killer, victim) = (killer.unwrap(), victim.unwrap());
                            for _ in 0..7 {
                                self.blood_particles
                                    .push(random_blood_particle(self.players[victim].position));
                            }
                            kill(&mut self.players, killer, victim);
                            self.text_scores[killer] =
                                score_text(ctx, self.players[killer].score, &mut self.assets)?;
                        }
                    }
                }
            }
        }

        self.fps.update(ctx, &self.assets, dt)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw(ctx, &self.fps.text, graphics::Point { x: 200.0, y: 10.0 }, 0.0)?;

        for i in 0..self.players.len() {
            draw_player(ctx, &mut self.players[i], &self.screen)?;
            let text_pos = graphics::Point {
                x: 20.0,
                y: i as f32 * 24.0 + 10.0,
            };
            graphics::draw(ctx, &self.text_scores[i], text_pos, 0.0)?;
        }

        for i in 0..self.blood_particles.len() {
            draw_blood(ctx,
                       &mut self.blood_particles[i],
                       &self.screen,
                       &mut self.assets)?;
        }

        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        for player in &mut self.players {
            if keycode == player.controls.up {
                player.input_axis.y = 1.0;
            } else if keycode == player.controls.left {
                player.facing = Facing::Left;
                player.input_axis.x = -1.0;
            } else if keycode == player.controls.right {
                player.facing = Facing::Right;
                player.input_axis.x = 1.0;
            }
        }
    }

    fn key_up_event(&mut self, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        for player in &mut self.players {
            if keycode == player.controls.up {
                player.input_axis.y = 0.0;
            } else if keycode == player.controls.left {
                if player.input_axis.x < 0. {
                    player.input_axis.x = 0.0;
                }
            } else if keycode == player.controls.right {
                if player.input_axis.x > 0. {
                    player.input_axis.x = 0.0;
                }
            }
        }
    }
}


pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "aknit".to_string();
    c.window_width = Screen::WIDTH;
    c.window_height = Screen::HEIGHT;
    let ctx = &mut Context::load_from_conf("aknit", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}