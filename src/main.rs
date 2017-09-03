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
use player::{Player, PlayerType};

mod controls;
mod display;
mod assets;
mod player;
mod animation;

// Main state
struct MainState {
    screen: Screen,
    assets: Assets,
    players: [Player; 2],
    text_scores: [graphics::Text; 2],
}

impl MainState {
    const DESIRED_FPS: u64 = 60;
    const GROUND_Y: f64 = 0.;
    const GRAVITY_MAGIC_NUMBER: f64 = 20.;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let text_scores = [graphics::Text::new(ctx, "0", &assets.font)?,
                           graphics::Text::new(ctx, "0", &assets.font)?];
        let player1 = Player::new(Controls {
                                      up: event::Keycode::Up,
                                      left: event::Keycode::Left,
                                      right: event::Keycode::Right,
                                  },
                                  PlayerType::Player1,
                                  assets::player1_animation(ctx)?);
        let player2 = Player::new(Controls {
                                      up: event::Keycode::E,
                                      left: event::Keycode::S,
                                      right: event::Keycode::F,
                                  },
                                  PlayerType::Player2,
                                  assets::player2_animation(ctx)?);
        let s = MainState {
            assets: assets,
            text_scores: text_scores,
            screen: Screen::new(),
            players: [player1, player2],
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        if !timer::check_update_time(ctx, Self::DESIRED_FPS) {
            return Ok(());
        }
        let seconds = 1.0 / (Self::DESIRED_FPS as f64);

        // Move players
        for player in &mut self.players {
            player.velocity.x = seconds * player.max_velocity.x * player.input_axis.x;

            if self.screen
                   .position_to_pixel(Vector2::new(0., Self::GROUND_Y))
                   .y as u32 >
               self.screen.position_to_pixel(player.position).y as u32 {
                player.velocity.y -= seconds * player.max_velocity.y / Self::GRAVITY_MAGIC_NUMBER;
            } else {
                player.velocity.y = seconds * player.max_velocity.y * player.input_axis.y;
            }

            player.position.x += player.velocity.x;
            player.position.y += player.velocity.y;

            if player.position.y < Self::GROUND_Y {
                player.position.y = Self::GROUND_Y;
            }
        }

        for i in 0..self.players.len() {
            let n = i + 1;
            for j in n..self.players.len() {
                // Check collision.
                if self.players[i]
                       .position
                       .distance(self.players[j].position) <
                   self.players[i].cbox_size.x {
                    if self.players[i].position.y > self.players[j].position.y &&
                       self.players[i].velocity.y < 0. {
                        kill(&mut self.players, i, j);
                        self.text_scores[i] =
                            score_text(ctx, self.players[i].score, &mut self.assets)?;
                    } else if self.players[j].position.y > self.players[i].position.y &&
                              self.players[j].velocity.y < 0. {
                        kill(&mut self.players, j, i);
                        self.text_scores[j] =
                            score_text(ctx, self.players[j].score, &mut self.assets)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for i in 0..self.players.len() {
            draw_player(ctx, &mut self.players[i], &self.screen)?;
            let text_pos = graphics::Point {
                x: 20.0,
                y: i as f32 * 24.0 + 10.0,
            };
            graphics::draw(ctx, &self.text_scores[i], text_pos, 0.0)?;
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
                player.input_axis.x = -1.0;
            } else if keycode == player.controls.right {
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

fn kill(players: &mut [Player; 2], killer_index: usize, victim_index: usize) {
    players[killer_index].score += 1;
    players[killer_index].velocity.y *= -0.7;
    players[victim_index].position = player::random_position();
    println!("{} killed {}", killer_index, victim_index);
}

fn score_text(ctx: &mut Context, score: u32, assets: &mut Assets) -> GameResult<graphics::Text> {
    graphics::Text::new(ctx, &format!("{}", score), &assets.font)
}

fn draw_player(ctx: &mut Context,
               player: &mut Player,
               screen: &Screen)
               -> GameResult<()> {
    let pixel_position = screen.position_to_pixel(player.position);
    let size = screen.size_to_pixel(player.size);
    let player_image = animation::player_image(player);

    graphics::draw_ex(ctx,
                      player_image,
                      graphics::DrawParam {
                          dest: graphics::Point {
                              x: pixel_position.x as f32,
                              y: pixel_position.y as f32,
                          },
                          scale: graphics::Point {
                              x: size.x as f32 / player_image.width() as f32,
                              y: size.x as f32 / player_image.height() as f32,
                          },
                          ..Default::default()
                      })?;
    Ok(())
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