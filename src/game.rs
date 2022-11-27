use sfml::graphics::{
    CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2f;

pub struct Game {
    _level: u8,
    window_width: u32,
    window_height: u32,
    mothership_pos_y: f32,
    mothership_pos_x: f32,
    mothership_direction: i8,
    mothership_width: u32,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32) -> Game {
        Game {
            _level: 1,
            window_width,
            window_height,
            mothership_pos_x: 50.0,
            mothership_pos_y: 100.0,
            mothership_direction: 10,
            mothership_width: 80,
        }
    }

    fn draw_mothership(&mut self, window: &mut RenderWindow) {
        let mut fin = RectangleShape::with_size(Vector2f::new(15.0, 15.0));
        fin.set_fill_color(Color::rgb(0, 255, 0));
        fin.set_position(Vector2f::new(
            self.mothership_pos_x,
            self.mothership_pos_y + 15.0,
        ));
        window.draw(&fin);
        fin.set_position(Vector2f::new(
            self.mothership_pos_x + 65.0,
            self.mothership_pos_y + 15.0,
        ));
        window.draw(&fin);
        let mut body = RectangleShape::with_size(Vector2f::new(50.0, 30.0));
        body.set_fill_color(Color::rgb(0, 255, 0));
        body.set_position(Vector2f::new(
            self.mothership_pos_x + 15.0,
            self.mothership_pos_y,
        ));
        window.draw(&body);
    }

    fn draw_ground(&mut self, window: &mut RenderWindow) {
        let mut ground = RectangleShape::with_size(Vector2f::new(self.window_width as f32, 40.0));
        ground.set_fill_color(Color::rgb(0, 128, 0));
        ground.set_position(Vector2f::new(0.0, self.window_height as f32 - 40.0));
        window.draw(&ground);
        let mut hill1 = CircleShape::new(150.0, 3);
        hill1.set_fill_color(Color::rgb(0, 128, 0));
        hill1.set_position(Vector2f::new(0.0, self.window_height as f32 - 150.0));
        window.draw(&hill1);
        let mut hill2 = CircleShape::new(300.0, 3);
        hill2.set_fill_color(Color::rgb(0, 128, 0));
        hill2.set_position(Vector2f::new(-300.0, self.window_height as f32 - 300.0));
        window.draw(&hill2);
        let mut hill3 = CircleShape::new(240.0, 3);
        hill3.set_fill_color(Color::rgb(0, 128, 0));
        hill3.set_position(Vector2f::new(
            self.window_width as f32 - 400.0,
            self.window_height as f32 - 240.0,
        ));
        window.draw(&hill3);
        let mut hill4 = CircleShape::new(340.0, 3);
        hill4.set_fill_color(Color::rgb(0, 128, 0));
        hill4.set_position(Vector2f::new(
            self.window_width as f32 - 370.0,
            self.window_height as f32 - 340.0,
        ));
        window.draw(&hill4);
    }

    fn draw_landing_pad(&mut self, window: &mut RenderWindow) {
        let pad_width = 250.0;
        let mut ground = RectangleShape::with_size(Vector2f::new(pad_width, 20.0));
        ground.set_fill_color(Color::rgb(0, 255, 0));
        ground.set_position(Vector2f::new(
            self.window_width as f32 / 2.0 - (pad_width / 2.0),
            self.window_height as f32 - 60.0,
        ));
        window.draw(&ground);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        self.draw_mothership(window);
        self.draw_ground(window);
        self.draw_landing_pad(window);
    }

    pub fn next_frame(&mut self) {
        if (self.mothership_pos_x > (self.window_width - (self.mothership_width + 50)) as f32
            && self.mothership_direction > 0)
            || (self.mothership_pos_x < 50.0 && self.mothership_direction < 0)
        {
            self.mothership_direction = -self.mothership_direction;
        }
        self.mothership_pos_x += self.mothership_direction as f32;
    }
}
