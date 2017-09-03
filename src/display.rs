use cgmath::Vector2;

// Screen
pub struct Screen {
    view_width: u32,
    view_height: u32,
}

impl Screen {
    pub const WIDTH: u32 = 320;
    pub const HEIGHT: u32 = 200;
    const RATIO: (u32, u32) = (8, 5);

    pub fn new() -> Screen {
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

    pub fn position_to_pixel(&self, postion: Vector2<f64>) -> Vector2<f64> {
        Vector2::new(self.view_width as f64 * postion.x + self.view_width as f64 / 2.,
                     self.view_height as f64 * -postion.y + self.view_height as f64 / 2.)

    }

    pub fn size_to_pixel(&self, size: Vector2<f64>) -> Vector2<f64> {
        Vector2::new(self.view_width as f64 * size.x,
                     self.view_height as f64 * size.y)

    }
}

