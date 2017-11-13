extern crate cgmath;
extern crate ggez;
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
use player::{Facing, Player, PlayerType};
use particles::Blood;
use helpers::*;
use bonus::{Bonus, BonusText};
use rand::{thread_rng, Rng, random};

mod controls;
mod display;
mod assets;
mod player;
mod animation;
mod particles;
mod helpers;
mod ui;
mod bonus;

const GROUND_Y: f64 = -0.33;
const GRAVITY_MAGIC_NUMBER: f64 = 20.;

enum Scene {
    Intro,
    Credits,
    Game,
}

// Main state
struct MainState {
    screen: Screen,
    assets: Assets,
    players: [Player; 2],
    text_scores: [graphics::Text; 2],
    blood_particles: Vec<Blood>,
    fps: Fps,
    scene: Scene,
    bonus_factory: bonus::Factory,
    bonuses: Vec<Bonus>,
    bonuses_text: Vec<BonusText>,
}

impl MainState {
    const DESIRED_FPS: u64 = 60;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        let assets = Assets::new(ctx)?;
        let text_scores = [
            graphics::Text::new(ctx, "0", &assets.font)?,
            graphics::Text::new(ctx, "0", &assets.font)?,
        ];
        let fps = Fps {
            text: graphics::Text::new(ctx, "FPS:", &assets.font)?,
            cooldown: 1.0,
        };
        let player1 = Player::new(
            Controls {
                up: event::Keycode::Up,
                left: event::Keycode::Left,
                right: event::Keycode::Right,
            },
            PlayerType::Player1,
            player1_animation(ctx)?,
        );
        let player2 = Player::new(
            Controls {
                up: event::Keycode::E,
                left: event::Keycode::S,
                right: event::Keycode::F,
            },
            PlayerType::Player2,
            player2_animation(ctx)?,
        );
        let s = MainState {
            assets: assets,
            text_scores: text_scores,
            screen: Screen::new(),
            players: [player1, player2],
            blood_particles: vec![],
            fps: fps,
            scene: Scene::Intro,
            bonus_factory: bonus::Factory::new(ctx)?,
            bonuses: vec![],
            bonuses_text: vec![],
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

        match self.scene {
            // Game Scene
            Scene::Game => {
                // Update players
                for player in &mut self.players {
                    player.update_mutations(dt);
                    player.update_position(&self.screen, seconds, &self.assets)?;
                }

                // Particles
                {
                    // Move
                    for blood_particle in &mut self.blood_particles {
                        blood_particle.velocity.y -= seconds * 1.2 / GRAVITY_MAGIC_NUMBER;
                        blood_particle.position.x += blood_particle.velocity.x;
                        blood_particle.position.y += blood_particle.velocity.y;
                    }
                    // Remove if out of screen
                    self.blood_particles
                        .retain(|blood_particle| blood_particle.position.y > -0.5);
                }

                // Bonus factory
                {
                    self.bonus_factory.position += self.bonus_factory.velocity * seconds;
                    if self.bonus_factory.position.x > 0.5 || self.bonus_factory.position.x < -0.5 {
                        self.bonus_factory.velocity.x *= -1.;
                    }
                }

                // Bonus text
                {
                    // Move
                    for bonus_text in &mut self.bonuses_text {
                        bonus_text.position.y += seconds * 0.1;
                        bonus_text.cooldown -= seconds;
                    }
                    // Remove if out of screen
                    self.bonuses_text
                        .retain(|bonus_text| bonus_text.cooldown > 0.);
                }

                // Bonus
                {
                    // Spawn
                    match self.bonus_factory.spawn(ctx, dt)? {
                        Some(bonus) => self.bonuses.push(bonus),
                        None => {}
                    }
                    // Move
                    for bonus in &mut self.bonuses {
                        bonus.rotation += (seconds * 500. * bonus.velocity.x) as f32;
                        if bonus.position.y > GROUND_Y - 0.03 {
                            bonus.velocity.y -= seconds / GRAVITY_MAGIC_NUMBER;
                        } else {
                            bonus.velocity.y *= -0.9;
                            if bonus.velocity.y < 0.01 {
                                bonus.velocity.x = 0.;
                            }
                        }
                        if bonus.position.x > 0.5 || bonus.position.x < -0.5 {
                            bonus.position.x *= -1.;
                        }
                        bonus.position.y += bonus.velocity.y;
                        bonus.position.x += bonus.velocity.x;
                    }
                }

                // Collision
                {
                    for i in 0..self.players.len() {
                        // Update cbox_size with bonuses
                        let mut cbox_size = self.players[i].cbox_size;
                        for m in &mut self.players[i].mutations {
                            cbox_size.x *= m.size_factor;
                            cbox_size.y *= m.size_factor;
                        }

                        // With other players
                        let n = i + 1;
                        for j in n..self.players.len() {
                            let distance =
                                self.players[i].position.distance(self.players[j].position);
                            if distance < cbox_size.x {
                                // This part should be updated.
                                // It seems cbox_size.y is never used nowhere.
                                // It must be used for better collisions.
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
                                        self.blood_particles.push(
                                            random_blood_particle(self.players[victim].position),
                                        );
                                    }
                                    kill(&mut self.players, killer, victim);
                                    self.text_scores[killer] = score_text(
                                        ctx,
                                        self.players[killer].score,
                                        &mut self.assets,
                                    )?;
                                    self.assets.death.play()?;
                                }
                            }
                        }
                        // With bonuses
                        for j in 0..self.bonuses.len() {
                            let bonus = &mut self.bonuses[j];
                            let distance = self.players[i].position.distance(bonus.position);
                            bonus.has_collision = distance <= cbox_size.x;
                            if bonus.has_collision {
                                self.assets.take_bonus.play()?;
                                self.bonuses_text.push(BonusText { 
                                    text: graphics::Text::new(ctx, &bonus.description, &self.assets.font_small)?,
                                    position: bonus.position,
                                    cooldown: 1.0,
                                });
                                match bonus.apply(&mut self.players[i]) {
                                    Some(m) => self.players[i].mutations.push(m),
                                    None => {
                                        self.text_scores[i] = score_text(
                                            ctx,
                                            self.players[i].score,
                                            &mut self.assets,
                                        )?;
                                    }
                                }
                            }
                        }
                        self.bonuses.retain(|bonus| !bonus.has_collision);
                    }
                }
            }
            _ => {}
        }

        self.fps.update(ctx, &self.assets, dt)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_full_screen(ctx, &self.assets.bg, &self.screen)?;

        match self.scene {
            // Game Scene
            Scene::Game => {

                // Images
                for i in 0..self.players.len() {
                    self.players[i].draw(ctx, &self.screen)?;
                }
                for i in 0..self.bonuses.len() {
                    self.bonuses[i].draw(ctx, &self.screen)?;
                }
                self.bonus_factory.draw(ctx, &self.screen)?;
                for i in 0..self.blood_particles.len() {
                    draw_blood(
                        ctx,
                        &mut self.blood_particles[i],
                        &self.screen,
                        &mut self.assets,
                    )?;
                }

                // Texts
                quick_draw(ctx, &self.text_scores[0], (0.4, 0.45), &self.screen)?;
                quick_draw(ctx, &self.text_scores[1], (-0.4, 0.45), &self.screen)?;
                quick_draw(ctx, &self.fps.text, (0., -0.45), &self.screen)?;
                for i in 0..self.bonuses_text.len() {
                    let a = (self.bonuses_text[i].position.x, self.bonuses_text[i].position.y);
                    quick_draw(ctx, &self.bonuses_text[i].text, a, &self.screen)?;
                }
            }
            Scene::Credits => {}

            // Intro Scene
            Scene::Intro => {
                graphics::set_color(ctx, graphics::Color::new(0., 0., 0., 0.75))?;
                let center = self.screen.position_to_pixel(Vector2::new(0., 0.));
                let size = self.screen.size_to_pixel(Vector2::new(1., 1.));
                let rect = graphics::Rect::new(
                    center.x as f32,
                    center.y as f32,
                    size.x as f32,
                    size.y as f32,
                );
                graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
                graphics::set_color(ctx, (255, 255, 255).into())?;

                for i in 0..self.assets.instructions_p1.len() {
                    let pos = self.screen
                        .position_to_pixel(Vector2::new(-0.15, 0.1 + i as f64 / -15.));
                    graphics::draw(
                        ctx,
                        &self.assets.instructions_p1[i],
                        vector2_to_point(pos),
                        0.,
                    )?;
                }
                for i in 0..self.assets.instructions_p2.len() {
                    let pos = self.screen
                        .position_to_pixel(Vector2::new(0.15, 0.1 + i as f64 / -15.));
                    graphics::draw(
                        ctx,
                        &self.assets.instructions_p2[i],
                        vector2_to_point(pos),
                        0.,
                    )?;
                }
                let title_pos = self.screen.position_to_pixel(Vector2::new(0., 0.4));
                let author_pos = self.screen.position_to_pixel(Vector2::new(0., 0.3));
                graphics::draw(ctx, &self.assets.title, vector2_to_point(title_pos), 0.)?;
                graphics::draw(ctx, &self.assets.authors, vector2_to_point(author_pos), 0.)?;
            }
        }

        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match self.scene {
            // Game Scene
            Scene::Game => for player in &mut self.players {
                if keycode == player.controls.up {
                    player.input_axis.y = 1.0;
                } else if keycode == player.controls.left {
                    player.facing = Facing::Left;
                    player.input_axis.x = -1.0;
                } else if keycode == player.controls.right {
                    player.facing = Facing::Right;
                    player.input_axis.x = 1.0;
                }
            },
            _ => if keycode == event::Keycode::Space {
                self.scene = Scene::Game;
            } else if keycode == event::Keycode::C {
                self.scene = Scene::Credits;
            },
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
