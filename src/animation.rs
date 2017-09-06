use ggez::graphics;
use player::Facing;

pub enum PlayerAnimationStatus {
    Walking,
    Standing,
    Jumping,
    Falling,
}

#[derive(Debug)]
pub struct LeftRightImage {
    pub left: graphics::Image,
    pub right: graphics::Image
}

impl LeftRightImage {
    pub fn face(&self, f: &Facing) -> &graphics::Image {
        match f {
            &Facing::Left => &self.left,
            &Facing::Right => &self.right,
        }
    }
}

#[derive(Debug)]
pub struct PlayerAnimation {
    pub stand: LeftRightImage,
    pub walk: [LeftRightImage; 2],
    pub jump: LeftRightImage,
    pub fall: LeftRightImage,
    pub time: f64,
}

impl PlayerAnimation {
    pub const WALK_ANIMATION_CYCLE: f64 = 0.2;
}