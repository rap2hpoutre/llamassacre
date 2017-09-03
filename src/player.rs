use cgmath::Vector2;
use controls::Controls;
use rand;
use animation::PlayerAnimation;

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
    pub size: Vector2<f64>,
    pub cbox_size: Vector2<f64>,
    pub max_velocity: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub input_axis: Vector2<f64>,
    pub controls: Controls,
    pub score: u32,
    pub animation: PlayerAnimation,
}

impl Player {
    pub fn new(controls: Controls, tag: PlayerType, animation: PlayerAnimation) -> Player {
        Player {
            tag: tag,
            position: random_position(),
            size: Vector2::new(0.05, 0.05),
            cbox_size: Vector2::new(0.025, 0.025),
            max_velocity: Vector2::new(0.2, 0.7),
            velocity: Vector2::new(0., 0.),
            input_axis: Vector2::new(0., 0.),
            controls: controls,
            score: 0,
            animation: animation,
        }
    }
}

pub fn random_position() -> Vector2<f64> {
    Vector2::new(rand::random::<f64>() - 0.5, 0.)
}