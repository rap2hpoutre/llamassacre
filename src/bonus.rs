struct Bonus {
    pub tag: PlayerType,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub cbox_size: Vector2<f64>,
    pub max_velocity: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub input_axis: Vector2<f64>,
    pub animation: BonusAnimation,
}