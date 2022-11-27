use sfml::graphics::{
    RenderTarget, RenderWindow, Shape, Transformable, Color, RectangleShape};
use sfml::system::Vector2f;

pub struct Game {
    _level: u8,
    window_width: u32,
    _window_height: u32,
    mothership_pos_y: f32,
    mothership_pos_x: f32,
    mothership_direction: i8,
    mothership_width: u32,
}

impl Game {
    pub fn new(window_width: u32, _window_height: u32) -> Game {
        Game{
            _level: 1,
            window_width,
            _window_height,
            mothership_pos_x: 50.0,
            mothership_pos_y: 50.0,
            mothership_direction: 20,
            mothership_width: 80,
        }
    }

    fn draw_mothership(&mut self, window: &mut RenderWindow) {
        let mut fin =  RectangleShape::with_size(Vector2f::new(15.0, 15.0));
        fin.set_fill_color(Color::rgb(0,255,0));
        fin.set_position(Vector2f::new(self.mothership_pos_x, self.mothership_pos_y + 15.0));
        window.draw(&fin);
        fin.set_position(Vector2f::new(self.mothership_pos_x + 65.0, self.mothership_pos_y + 15.0));
        window.draw(&fin);
        let mut body =  RectangleShape::with_size(Vector2f::new(50.0, 30.0));
        body.set_fill_color(Color::rgb(0,255,0));
        body.set_position(Vector2f::new(self.mothership_pos_x + 15.0, self.mothership_pos_y));
        window.draw(&body);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        self.draw_mothership(window);
    }

    pub fn iterate(&mut self) {
        if (self.mothership_pos_x > (self.window_width - (self.mothership_width + 50)) as f32 &&
                self.mothership_direction > 0)||
           (self.mothership_pos_x < 50.0 && self.mothership_direction < 0 ) {
            self.mothership_direction = -self.mothership_direction;
        }
        self.mothership_pos_x += self.mothership_direction as f32;
    }
}