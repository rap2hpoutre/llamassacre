extern crate ggez;
extern crate cgmath;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::timer;
use std::time::Duration;
use cgmath::Vector2;

// Screen
struct Screen {
    view_width: u32,
    view_height: u32,
}

impl Screen {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 200;
    const RATIO: (u32, u32) = (8, 5);

    fn new() -> Screen {
        let cell_by_width = Screen::WIDTH as f32 / Screen::RATIO.0 as f32;
        let cell_by_height = Screen::HEIGHT as f32 / Screen::RATIO.1 as f32;
        let cell_size = if cell_by_width > cell_by_height {
            cell_by_width
        } else {
            cell_by_width
        };
        let view_width = (cell_size * Screen::RATIO.0 as f32) as u32;
        let view_height = (cell_size * Screen::RATIO.1 as f32) as u32;
        println!("View size: {} {}", view_width, view_height);
        Screen {
            view_width: view_width,
            view_height: view_height,
        }
    }

    fn position_to_pixel(&self, postion: Vector2<f64>) -> Vector2<f64> {
        Vector2::new(self.view_width as f64 * postion.x + self.view_width as f64 / 2.,
                     self.view_height as f64 * -postion.y + self.view_height as f64 / 2.)

    }

    fn size_to_pixel(&self, size: Vector2<f64>) -> Vector2<f64> {
        Vector2::new(self.view_width as f64 * size.x,
                     self.view_height as f64 * size.y)

    }
}

// Assets
struct Assets {
    player_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let color = graphics::Color::new(0.5, 0., 0., 1.);
        let s = Assets { player_image: graphics::Image::solid(ctx, 22, color)? };
        Ok(s)
    }

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Player => &mut self.player_image,
        }
    }
}

// Controls
#[derive(Debug)]
struct Controls {
    up: event::Keycode,
    left: event::Keycode,
    right: event::Keycode,
}


// Actors
#[derive(Debug)]
enum ActorType {
    Player,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    position: Vector2<f64>,
    size: Vector2<f64>,
    cbox_size: Vector2<f64>,
    max_velocity: Vector2<f64>,
    velocity: Vector2<f64>,
    input_axis: Vector2<f64>,
    controls: Controls,
}

impl Actor {
    fn new_player(controls: Controls) -> Actor {
        Actor {
            tag: ActorType::Player,
            position: Vector2::new(0., 0.),
            size: Vector2::new(0.05, 0.05),
            cbox_size: Vector2::new(0.025, 0.025),
            max_velocity: Vector2::new(0.2, 0.7),
            velocity: Vector2::new(0., 0.),
            input_axis: Vector2::new(0., 0.),
            controls: controls,
        }
    }
}

// Main state
struct MainState {
    screen: Screen,
    assets: Assets,
    players: [Actor; 2],
}

impl MainState {
    const DESIRED_FPS: u64 = 60;
    const GROUND_Y: f64 = 0.;
    const GRAVITY_MAGIC_NUMBER: f64 = 20.;

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let s = MainState {
            assets: assets,
            screen: Screen::new(),
            players: [Actor::new_player(Controls {
                                            up: event::Keycode::Up,
                                            left: event::Keycode::Left,
                                            right: event::Keycode::Right,
                                        }),
                      Actor::new_player(Controls {
                                            up: event::Keycode::E,
                                            left: event::Keycode::S,
                                            right: event::Keycode::F,
                                        })],
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
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_actor(ctx, &self.players[0], &mut self.assets, &self.screen)?;
        draw_actor(ctx, &self.players[1], &mut self.assets, &self.screen)?;
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

fn draw_actor(ctx: &mut Context,
              actor: &Actor,
              assets: &mut Assets,
              screen: &Screen)
              -> GameResult<()> {
    let pixel_position = screen.position_to_pixel(actor.position);
    let size = screen.size_to_pixel(actor.size);
    let actor_image = assets.actor_image(actor);

    graphics::draw_ex(ctx,
                      actor_image,
                      graphics::DrawParam {
                          dest: graphics::Point {
                              x: pixel_position.x as f32,
                              y: pixel_position.y as f32,
                          },
                          scale: graphics::Point {
                              x: size.x as f32 / actor_image.width() as f32,
                              y: size.x as f32 / actor_image.height() as f32,
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