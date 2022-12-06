use rand::Rng;
use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::Vector2f;
use sfml::SfBox;

struct Asteroid {
    height: u32,
    x_pos: i32,
    speed: i32,
    r1: f32,
    r2: f32,
    r3: f32,
}

#[derive(Eq, PartialEq)]
enum PodStatus {
    Inactive,
    Dropping,
    Landed,
    _Ascending,
    Exploding,
    ReadyForTakeOff,
}

pub enum PodMove {
    Left,
    Right,
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
    font: SfBox<Font>,
    ground_height: f32,
    landing_pad_height: f32,
    landing_pad_width: f32,
    landing_pad_x: f32,
    pod_size: f32,
    pod_status: PodStatus,
    pod_explosion_timer: u8,
    pub pod_new_explosion: bool,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32, resource_path: String) -> Game {
        let pad_width = 250.0;
        let font = Font::from_file(&(resource_path + "/zx-spectrum.ttf")).unwrap();
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
            font,
            ground_height: 40.0,
            landing_pad_height: 20.0,
            landing_pad_width: pad_width,
            landing_pad_x: window_width as f32 / 2.0 - (pad_width / 2.0),
            pod_size: 20.0,
            pod_status: PodStatus::Inactive,
            pod_explosion_timer: 0,
            pod_new_explosion: true,
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
        let ground_colour = 96;
        let hill_colour = 64;
        let mut hill1 = CircleShape::new(150.0, 3);
        hill1.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill1.set_position(Vector2f::new(0.0, self.window_height as f32 - 150.0));
        window.draw(&hill1);
        let mut hill2 = CircleShape::new(300.0, 3);
        hill2.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill2.set_position(Vector2f::new(-300.0, self.window_height as f32 - 300.0));
        window.draw(&hill2);
        let mut hill3 = CircleShape::new(240.0, 3);
        hill3.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill3.set_position(Vector2f::new(
            self.window_width as f32 - 400.0,
            self.window_height as f32 - 240.0,
        ));
        window.draw(&hill3);
        let mut hill4 = CircleShape::new(340.0, 3);
        hill4.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill4.set_position(Vector2f::new(
            self.window_width as f32 - 370.0,
            self.window_height as f32 - 340.0,
        ));
        window.draw(&hill4);
        let mut ground =
            RectangleShape::with_size(Vector2f::new(self.window_width as f32, self.ground_height));
        ground.set_fill_color(Color::rgb(0, ground_colour, 0));
        ground.set_position(Vector2f::new(0.0, self.window_height as f32 - 40.0));
        window.draw(&ground);
    }

    fn draw_landing_pad(&mut self, window: &mut RenderWindow) {
        let mut pad = RectangleShape::with_size(Vector2f::new(
            self.landing_pad_width,
            self.landing_pad_height,
        ));
        pad.set_fill_color(Color::rgb(0, 200, 0));
        pad.set_position(Vector2f::new(
            self.landing_pad_x,
            self.window_height as f32 - self.ground_height - self.landing_pad_height,
        ));
        window.draw(&pad);
    }

    fn draw_moonbase(&mut self, window: &mut RenderWindow) {
        let mut moonbase = CircleShape::new(100.0, 32);
        moonbase.set_fill_color(Color::rgb(0, 110, 0));
        moonbase.set_position(Vector2f::new(
            self.window_width as f32 * 0.75,
            self.window_height as f32 - 100.0 - self.ground_height,
        ));
        window.draw(&moonbase);
    }

    fn draw_asteroids(&mut self, window: &mut RenderWindow) {
        for asteroid in &self.asteroids {
            let mut blob3 = CircleShape::new(asteroid.r3, 8);
            blob3.set_fill_color(Color::rgb(0, 80, 0));
            blob3.set_position(Vector2f::new(
                asteroid.x_pos as f32 + 60.0,
                asteroid.height as f32 + 10.0,
            ));
            window.draw(&blob3);
            let mut blob2 = CircleShape::new(asteroid.r2, 8);
            blob2.set_fill_color(Color::rgb(0, 100, 0));
            blob2.set_position(Vector2f::new(
                asteroid.x_pos as f32 + 20.0,
                asteroid.height as f32,
            ));
            window.draw(&blob2);
            let mut blob1 = CircleShape::new(asteroid.r1, 8);
            blob1.set_fill_color(Color::rgb(0, 120, 0));
            blob1.set_position(Vector2f::new(
                asteroid.x_pos as f32,
                asteroid.height as f32 + 10.0,
            ));
            window.draw(&blob1);
        }
    }

    fn draw_pod(&mut self, window: &mut RenderWindow) {
        if self.pod_status == PodStatus::Exploding {
            let mut rng = rand::thread_rng();
            let radius = rng.gen_range(20.0..200.0);
            let mut explosion = CircleShape::new(radius, 32);
            let lum = rng.gen_range(200..255);
            explosion.set_fill_color(Color::rgb(0, lum, 0));
            explosion.set_position(Vector2f::new(
                self.pod_pos_x as f32 - radius + self.pod_size / 2.0,
                self.pod_pos_y as f32 - radius + self.pod_size / 2.0,
            ));
            window.draw(&explosion);
            self.pod_explosion_timer += 1;
            if self.pod_explosion_timer > 20 {
                self.pod_explosion_timer = 0;
                self.pod_status = PodStatus::Inactive;
            }
            return;
        }
        let mut pod = RectangleShape::with_size(Vector2f::new(self.pod_size, self.pod_size));
        pod.set_fill_color(Color::rgb(0, 255, 0));
        pod.set_position(Vector2f::new(self.pod_pos_x, self.pod_pos_y));
        window.draw(&pod);
    }

    fn draw_text(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!(
                "Xtarda Rescue!   Rescued: {} / {}   Pods Remaining: {}",
                0, 10, 3
            ),
            &self.font,
            30,
        );
        text.set_position(Vector2f::new(20.0, 20.0));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        self.draw_mothership(window);
        self.draw_moonbase(window);
        self.draw_ground(window);
        self.draw_landing_pad(window);
        self.draw_asteroids(window);
        self.draw_text(window);
        if self.pod_status != PodStatus::Inactive {
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
        if self.pod_status == PodStatus::Dropping {
            if !self.check_for_pod_landing() {
                if self.check_for_pod_collision() {
                    self.explode_pod();
                }
                self.pod_pos_y += 5.0;
            }
        }
    }

    fn explode_pod(&mut self) {
        self.pod_status = PodStatus::Exploding;
        self.pod_new_explosion = true;
    }

    pub fn is_pod_landed(&self) -> bool {
        return self.pod_status == PodStatus::Landed;
    }

    pub fn is_pod_exploding(&self) -> bool {
        return self.pod_status == PodStatus::Exploding;
    }

    fn check_for_pod_landing(&mut self) -> bool {
        if self.pod_pos_y
            >= self.window_height as f32
                - self.ground_height
                - self.landing_pad_height
                - self.pod_size
            && self.pod_pos_x >= self.landing_pad_x - self.pod_size / 2.0
            && self.pod_pos_x <= self.landing_pad_x + self.landing_pad_width - self.pod_size / 2.0
        {
            self.pod_pos_y = self.window_height as f32
                - self.ground_height
                - self.landing_pad_height
                - self.pod_size;
            self.pod_status = PodStatus::Landed;
            return true;
        }
        if self.pod_pos_y >= self.window_height as f32 - self.ground_height - self.pod_size {
            self.pod_pos_y = self.window_height as f32 - self.ground_height - self.pod_size;
            self.explode_pod();
            return true;
        }
        false
    }

    fn check_for_pod_collision(&mut self) -> bool {
        for asteroid in &self.asteroids {
            // This is very rudimentary, TODO improve bounding box
            // But having said that, it seems to work well :)
            if self.pod_pos_x >= asteroid.x_pos as f32 - self.pod_size
                && self.pod_pos_x <= asteroid.x_pos as f32 + 120.0
                && self.pod_pos_y >= asteroid.height as f32
                && self.pod_pos_y <= asteroid.height as f32 + 30.0
            {
                return true;
            }
        }
        return false;
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
        self.asteroids.clear();
        let num_asteroids = 20 + 2 * level;
        let mut rng = rand::thread_rng();
        for n in 0..num_asteroids {
            let max_speed = (5 + level * 2) as i32;
            let mut speed = rng.gen_range(-max_speed..max_speed);
            if speed == 0 {
                speed = max_speed;
            }
            let asteroid = Asteroid {
                height: ((self.window_height as f32 * 0.144)
                    + (self.window_height as f32 * 0.02) * n as f32) as u32,
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
        if self.pod_status == PodStatus::Dropping {
            return;
        };
        self.pod_status = PodStatus::Dropping;
        self.pod_pos_x = self.mothership_pos_x + 40.0;
        self.pod_pos_y = self.mothership_pos_y + 30.0;
    }

    pub fn set_pod_ready_for_take_off(&mut self) {
        self.pod_status = PodStatus::ReadyForTakeOff;
    }

    pub fn pod_manoeuvre(&mut self, direction: PodMove) {
        if self.pod_status == PodStatus::Dropping || self.pod_status == PodStatus::_Ascending {
            match direction {
                PodMove::Left => {
                    self.pod_pos_x -= 4.0;
                }
                PodMove::Right => {
                    self.pod_pos_x += 4.0;
                }
            }
        }
    }
}
