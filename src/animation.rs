use ggez::graphics;
use Player;

pub enum PlayerAnimationStatus {
    Walking,
    Standing,
    Jumping,
    Falling,
}

#[derive(Debug)]
pub struct PlayerAnimation {
    pub stand: graphics::Image,
    pub walk: [graphics::Image; 2],
    pub jump: graphics::Image,
    pub fall: graphics::Image,
    pub time: f64,
}

impl PlayerAnimation {
    pub const WALK_ANIMATION_CYCLE: f64 = 0.2;
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
                player.animation.time += 0.017; // TODO
                if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE * 2. {
                    player.animation.time = 0.;
                    &player.animation.walk[0]
                } else if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE {
                    &player.animation.walk[0]
                } else {
                    &player.animation.walk[1]
                }
            },
        PlayerAnimationStatus::Standing => &player.animation.stand,
        PlayerAnimationStatus::Jumping => &player.animation.jump,
        PlayerAnimationStatus::Falling => &player.animation.fall, 
    }
}