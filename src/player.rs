use cgmath::Vector2;
use controls::Controls;
use animation::PlayerAnimation;

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
        }
    }
}