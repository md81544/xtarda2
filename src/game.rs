use rand::Rng;
use sfml::graphics::{
    CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2f;

struct Asteroid {
    height: u32,
    x_pos: i32,
    speed: i32,
    r1: f32,
    r2: f32,
    r3: f32,
}

pub struct Game {
    level: u8,
    window_width: u32,
    window_height: u32,
    mothership_pos_y: f32,
    mothership_pos_x: f32,
    mothership_direction: i8,
    mothership_width: u32,
    asteroids: Vec<Asteroid>,
    pod_pos_x: f32,
    pod_pos_y: f32,
    pod_dropping: bool,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32) -> Game {
        Game {
            level: 1,
            window_width,
            window_height,
            mothership_pos_x: 50.0,
            mothership_pos_y: 100.0,
            mothership_direction: 10,
            mothership_width: 80,
            asteroids: Vec::new(),
            pod_pos_x: 0.0,
            pod_pos_y: 100.0,
            pod_dropping: false,
        }
    }

    fn draw_mothership(&mut self, window: &mut RenderWindow) {
        let mut fin = RectangleShape::with_size(Vector2f::new(15.0, 15.0));
        fin.set_fill_color(Color::rgb(0, 200, 0));
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
        let mut pad = RectangleShape::with_size(Vector2f::new(pad_width, 20.0));
        pad.set_fill_color(Color::rgb(0, 200, 0));
        pad.set_position(Vector2f::new(
            self.window_width as f32 / 2.0 - (pad_width / 2.0),
            self.window_height as f32 - 60.0,
        ));
        window.draw(&pad);
    }

    fn draw_asteroids(&mut self, window: &mut RenderWindow) {
        for asteroid in &self.asteroids {
            let mut blob1 = CircleShape::new(asteroid.r1, 8);
            blob1.set_fill_color(Color::rgb(0, 120, 0));
            blob1.set_position(Vector2f::new(
                asteroid.x_pos as f32,
                asteroid.height as f32 + 10.0,
            ));
            window.draw(&blob1);
            let mut blob2 = CircleShape::new(asteroid.r2, 8);
            blob2.set_fill_color(Color::rgb(0, 100, 0));
            blob2.set_position(Vector2f::new(
                asteroid.x_pos as f32 + 20.0,
                asteroid.height as f32,
            ));
            window.draw(&blob2);
            let mut blob3 = CircleShape::new(asteroid.r3, 8);
            blob3.set_fill_color(Color::rgb(0, 80, 0));
            blob3.set_position(Vector2f::new(
                asteroid.x_pos as f32 + 60.0,
                asteroid.height as f32 + 10.0,
            ));
            window.draw(&blob3);
        }
    }

    fn draw_pod(&mut self, window: &mut RenderWindow) {
        let mut pod = RectangleShape::with_size(Vector2f::new(20.0, 20.0));
        pod.set_fill_color(Color::rgb(0, 255, 0));
        pod.set_position(Vector2f::new(self.pod_pos_x, self.pod_pos_y));
        window.draw(&pod);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        self.draw_mothership(window);
        self.draw_ground(window);
        self.draw_landing_pad(window);
        self.draw_asteroids(window);
        if self.pod_dropping {
            self.draw_pod(window);
        }
    }

    pub fn next_frame(&mut self) {
        if (self.mothership_pos_x > (self.window_width - (self.mothership_width + 50)) as f32
            && self.mothership_direction > 0)
            || (self.mothership_pos_x < 50.0 && self.mothership_direction < 0)
        {
            self.mothership_direction = -self.mothership_direction;
        }
        self.mothership_pos_x += self.mothership_direction as f32;
        for asteroid in &mut self.asteroids {
            asteroid.x_pos += asteroid.speed;
            if asteroid.speed > 0 && asteroid.x_pos > self.window_width as i32 {
                asteroid.x_pos = -150;
            }
            if asteroid.speed < 0 && asteroid.x_pos < -150 {
                asteroid.x_pos = self.window_width as i32;
            }
        }
        if self.pod_dropping {
            self.pod_pos_y += 5.0;
            if self.pod_pos_y > self.window_height as f32 {
                self.pod_dropping = false;
            }
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
        self.asteroids.clear();
        let num_asteroids = 5 + 2 * level;
        let mut rng = rand::thread_rng();
        for n in 0..num_asteroids {
            let max_speed = (4 + level * 2) as i32;
            let mut speed = rng.gen_range(-max_speed..max_speed);
            if speed == 0 {
                speed = max_speed;
            }
            let asteroid = Asteroid {
                height: 180 + 50 * n as u32,
                x_pos: rng.gen_range(50..self.window_width as i32 - 50),
                speed: speed,
                r1: rng.gen_range(20.0..40.0),
                r2: rng.gen_range(30.0..50.0),
                r3: rng.gen_range(20.0..40.0),
            };
            self.asteroids.push(asteroid);
        }
    }

    pub fn drop_pod(&mut self) {
        if self.pod_dropping == true {
            return;
        };
        self.pod_dropping = true;
        self.pod_pos_x = self.mothership_pos_x + 40.0;
        self.pod_pos_y = self.mothership_pos_y + 30.0;
    }
}
