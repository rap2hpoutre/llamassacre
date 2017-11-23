use ggez::graphics;
use ggez::{Context, GameResult};
use cgmath::Vector2;
use animation::{LeftRightImage, PlayerAnimation, PlayerAnimationStatus};
use player::Player;
use assets::Assets;
use display::Screen;
use particles::Blood;
use rand;



pub fn player1_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            LeftRightImage {
                left: sprite(ctx, "/sprite_02.png")?,
                right: sprite(ctx, "/sprite_07.png")?,
            },
            LeftRightImage {
                left: sprite(ctx, "/sprite_03.png")?,
                right: sprite(ctx, "/sprite_08.png")?,
            },
        ],
        jump: LeftRightImage {
            left: sprite(ctx, "/sprite_04.png")?,
            right: sprite(ctx, "/sprite_09.png")?,
        },
        fall: LeftRightImage {
            left: sprite(ctx, "/sprite_05.png")?,
            right: sprite(ctx, "/sprite_10.png")?,
        },
        stand: LeftRightImage {
            left: sprite(ctx, "/sprite_01.png")?,
            right: sprite(ctx, "/sprite_06.png")?,
        },
        time: 0.,
    };
    Ok(r)
}

pub fn player2_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            LeftRightImage {
                left: sprite(ctx, "/sprite_13.png")?,
                right: sprite(ctx, "/sprite_18.png")?,
            },
            LeftRightImage {
                left: sprite(ctx, "/sprite_14.png")?,
                right: sprite(ctx, "/sprite_19.png")?,
            },
        ],
        jump: LeftRightImage {
            left: sprite(ctx, "/sprite_15.png")?,
            right: sprite(ctx, "/sprite_20.png")?,
        },
        fall: LeftRightImage {
            left: sprite(ctx, "/sprite_16.png")?,
            right: sprite(ctx, "/sprite_21.png")?,
        },
        stand: LeftRightImage {
            left: sprite(ctx, "/sprite_12.png")?,
            right: sprite(ctx, "/sprite_17.png")?,
        },
        time: 0.,
    };
    Ok(r)
}

pub fn sprite(ctx: &mut Context, s: &str) -> GameResult<graphics::Image> {
    let mut llama_s = graphics::Image::new(ctx, s)?;
    llama_s.set_filter(graphics::FilterMode::Nearest);
    Ok(llama_s)
}

pub fn player_image(player: &mut Player) -> &graphics::Image {
    let status = if player.velocity.y < 0. {
        PlayerAnimationStatus::Falling
    } else if player.velocity.y > 0. {
        PlayerAnimationStatus::Jumping
    } else if player.velocity.x != 0. {
        PlayerAnimationStatus::Walking
    } else {
        PlayerAnimationStatus::Standing
    };
    match status {
        PlayerAnimationStatus::Walking => {
            player.animation.time += 1. / 60.; // TODO
            if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE * 2. {
                player.animation.time = 0.;
                &player.animation.walk[0].face(&player.facing)
            } else if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE {
                &player.animation.walk[0].face(&player.facing)
            } else {
                &player.animation.walk[1].face(&player.facing)
            }
        }
        PlayerAnimationStatus::Standing => &player.animation.stand.face(&player.facing),
        PlayerAnimationStatus::Jumping => &player.animation.jump.face(&player.facing),
        PlayerAnimationStatus::Falling => &player.animation.fall.face(&player.facing),
    }
}

pub fn kill(players: &mut [Player; 2], killer_index: usize, victim_index: usize) {
    players[killer_index].score += 1;
    players[killer_index].velocity.y *= -1.0;
    players[victim_index].position = random_position();
}

pub fn score_text(
    ctx: &mut Context,
    score: u32,
    assets: &mut Assets,
) -> GameResult<graphics::Text> {
    graphics::Text::new(ctx, &format!("{}", score), &assets.font)
}

pub fn quick_draw(
    ctx: &mut Context,
    d: &graphics::Drawable,
    position: (f64, f64),
    screen: &Screen,
) -> GameResult<()> {
    let p = point_from_position(Vector2::new(position.0, position.1), screen);
    Ok(graphics::draw(ctx, d, p, 0.)?)
}

pub fn draw_blood(
    ctx: &mut Context,
    blood: &Blood,
    screen: &Screen,
    assets: &mut Assets,
) -> GameResult<()> {
    let blood_image = &assets.blood;

    graphics::draw_ex(
        ctx,
        blood_image,
        graphics::DrawParam {
            dest: point_from_position(blood.position, screen),
            scale: scale(blood.size, screen, blood_image),
            rotation: blood.velocity.y.atan2(blood.velocity.x * -1.0) as f32,
            ..Default::default()
        },
    )?;
    Ok(())
}

pub fn point_from_position(position: Vector2<f64>, screen: &Screen) -> graphics::Point {
    vector2_to_point(screen.position_to_pixel(position))
}

pub fn vector2_to_point(v: Vector2<f64>) -> graphics::Point {
    graphics::Point {
        x: v.x as f32,
        y: v.y as f32,
    }
}

pub fn scale(size: Vector2<f64>, screen: &Screen, image: &graphics::Image) -> graphics::Point {
    let size = screen.size_to_pixel(size);
    graphics::Point {
        x: size.x as f32 / image.width() as f32,
        y: size.x as f32 / image.height() as f32,
    }
}

pub fn random_position() -> Vector2<f64> {
    Vector2::new(rand::random::<f64>() - 0.5, ::GROUND_Y)
}

pub fn random_blood_particle(position: Vector2<f64>) -> Blood {
    Blood {
        position: position,
        size: Vector2::new(0.025, 0.025),
        velocity: Vector2::new(
            (rand::random::<f64>() - 0.5) / 50.,
            rand::random::<f64>() / 40.,
        ),
    }
}

pub fn draw_full_screen(
    ctx: &mut Context,
    image: &graphics::Image,
    screen: &Screen,
) -> GameResult<()> {
    let dest = point_from_position(Vector2::new(0., 0.), screen);
    let size = screen.size_to_pixel(Vector2::new(1., 1.));
    let draw_param = graphics::DrawParam {
        dest: dest,
        scale: graphics::Point {
            x: size.x as f32 / image.width() as f32,
            y: size.y as f32 / image.height() as f32,
        },
        ..Default::default()
    };
    graphics::draw_ex(ctx, image, draw_param)?;
    Ok(())
}

pub fn is_on_top(first: Vector2<f64>, second: Vector2<f64>, screen: &Screen) -> bool {
    screen.position_to_pixel(second).y as i32 > screen.position_to_pixel(first).y as i32
}

pub fn transparent_layer(ctx: &mut Context, screen: &Screen) -> GameResult<()> {
    graphics::set_color(ctx, graphics::Color::new(0., 0., 0., 0.75))?;
    let center = screen.position_to_pixel(Vector2::new(0., 0.));
    let size = screen.size_to_pixel(Vector2::new(1., 1.));
    let rect = graphics::Rect::new(
        center.x as f32,
        center.y as f32,
        size.x as f32,
        size.y as f32,
    );
    graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
    graphics::set_color(ctx, (255, 255, 255).into())?;
    Ok(())
}