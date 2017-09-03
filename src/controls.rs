use ggez::event;

// Controls
#[derive(Debug)]
pub struct Controls {
    pub up: event::Keycode,
    pub left: event::Keycode,
    pub right: event::Keycode,
}