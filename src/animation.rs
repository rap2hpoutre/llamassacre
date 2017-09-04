use ggez::graphics;
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