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
enum ManStatus {
    Inactive,
    EnteringPod,
    _Dropping,
}

#[derive(Eq, PartialEq)]
enum PodStatus {
    Inactive,
    Dropping,
    Landed,
    Ascending,
    Exploding,
    ReadyForTakeOff,
    AutoDock,
}

pub enum PodMove {
    Left,
    Right,
}

pub enum Sounds {
    Explosion,
    Landed,
    Docked,
}

#[derive(Eq, PartialEq)]
pub enum GameStatus {
    Playing,
    SplashScreen,
    GameOver,
    NewLevel,
}

pub struct Game {
    pub game_status: GameStatus,
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
    pub sounds_to_play: Vec<Sounds>,
    men_to_rescue: u32,
    pods_remaining: u32,
    man_pos_x: f32,
    man_pos_y: f32,
    man_status: ManStatus,
}

impl Game {
    pub fn new(window_width: u32, window_height: u32, resource_path: String) -> Game {
        let pad_width = 250.0;
        let font = Font::from_file(&(resource_path + "/zx-spectrum.ttf")).unwrap();
        Game {
            game_status: GameStatus::SplashScreen,
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
            sounds_to_play: vec![],
            men_to_rescue: 5,
            pods_remaining: 0,
            man_pos_x: window_width as f32 * 0.75,
            man_pos_y: window_height as f32 - 60.0,
            man_status: ManStatus::Inactive,
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

    fn draw_man(&mut self, window: &mut RenderWindow) {
        if self.man_status == ManStatus::Inactive {
            return;
        }
        let mut man = RectangleShape::with_size(Vector2f::new(6.0, 20.0));
        man.set_fill_color(Color::rgb(0, 255, 0));
        man.set_position(Vector2f::new(self.man_pos_x, self.man_pos_y));
        window.draw(&man);
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
        pad.set_fill_color(Color::rgb(0, 120, 0));
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
                "Level: {}  Terrans to Rescue: {}  Pods Left: {}",
                self.level, self.men_to_rescue, self.pods_remaining
            ),
            &self.font,
            (self.window_width as f32 * 0.015625) as u32,
        );
        text.set_position(Vector2f::new(150.0, 20.0));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
    }
    fn draw_splash_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!("Xtarda Rescue!"),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(150.0, 200.0));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_press_enter(window);
    }

    fn draw_new_level_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!("Level {}", self.level),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(150.0, 200.0));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_press_enter(window);
    }

    fn draw_game_over_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!("Game Over"),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(150.0, 200.0));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_press_enter(window);
    }

    fn draw_press_enter(&self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!("Press ENTER to continue"),
            &self.font,
            (self.window_width as f32 * 0.03) as u32,
        );
        text.set_position(Vector2f::new(150.0, 500.0));
        text.set_fill_color(Color::rgb(0, 150, 0));
        window.draw(&text);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        match self.game_status {
            GameStatus::Playing => {
                self.draw_mothership(window);
                self.draw_moonbase(window);
                self.draw_landing_pad(window);
                self.draw_man(window);
                self.draw_ground(window);
                self.draw_asteroids(window);
                self.draw_text(window);
                if self.pod_status != PodStatus::Inactive {
                    self.draw_pod(window);
                }
            }
            GameStatus::SplashScreen => {
                self.draw_splash_screen(window);
            }
            GameStatus::NewLevel => {
                self.draw_new_level_screen(window);
            }
            GameStatus::GameOver => {
                self.draw_game_over_screen(window);
            }
        }
    }

    pub fn next_frame(&mut self) {
        if self.game_status != GameStatus::Playing {
            return;
        }
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
        if self.pod_status == PodStatus::Ascending {
            if !self.check_for_pod_docking() {
                if self.check_for_pod_collision() {
                    self.explode_pod();
                }
                self.pod_pos_y -= 5.0;
            }
        }
        if self.pod_status == PodStatus::AutoDock {
            if self.mothership_pos_x < self.pod_pos_x {
                self.pod_pos_x -= 20.0;
            } else {
                self.pod_pos_x += 20.0;
            }
            self.check_for_pod_docking();
        }
        if self.man_status == ManStatus::EnteringPod {
            if self.man_pos_x > self.pod_pos_x + 15.0 {
                self.man_pos_x -= 10.0;
            } else {
                self.man_status = ManStatus::Inactive;
                self.man_pos_x = self.window_width as f32 * 0.75;
                self.man_pos_y = self.window_height as f32 - 60.0;
            }
        }
    }

    fn explode_pod(&mut self) {
        self.pod_status = PodStatus::Exploding;
        self.sounds_to_play.push(Sounds::Explosion);
        self.pods_remaining -= 1;
        if self.pods_remaining == 0 {
            self.game_over();
        }
    }

    fn game_over(&mut self) {
        self.game_status = GameStatus::GameOver;
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
            self.pod_status = PodStatus::ReadyForTakeOff;
            self.sounds_to_play.push(Sounds::Landed);
            self.man_status = ManStatus::EnteringPod;
            return true;
        }
        if self.pod_pos_y >= self.window_height as f32 - self.ground_height - self.pod_size {
            self.pod_pos_y = self.window_height as f32 - self.ground_height - self.pod_size;
            self.explode_pod();
            return true;
        }
        false
    }

    fn check_for_pod_docking(&mut self) -> bool {
        if self.pod_pos_y <= self.mothership_pos_y + 10.0 {
            if self.pod_pos_x >= self.mothership_pos_x
                && self.pod_pos_x
                    <= self.mothership_pos_x + self.mothership_width as f32 - self.pod_size
            {
                self.sounds_to_play.push(Sounds::Docked);
                self.pod_status = PodStatus::Inactive;
                self.men_to_rescue -= 1;
                if self.men_to_rescue == 0 {
                    // TODO some form of congratulation / next level
                    self.set_level(self.level + 1);
                }
                return true;
            } else {
                self.pod_status = PodStatus::AutoDock;
            }
        }
        return false;
    }

    fn check_for_pod_collision(&mut self) -> bool {
        for (idx, asteroid) in self.asteroids.iter().enumerate() {
            // This is very rudimentary, TODO improve bounding box
            // But having said that, it seems to work well :)
            if self.pod_pos_x >= asteroid.x_pos as f32 - self.pod_size
                && self.pod_pos_x <= asteroid.x_pos as f32 + 120.0
                && self.pod_pos_y >= asteroid.height as f32
                && self.pod_pos_y <= asteroid.height as f32 + 30.0
            {
                self.asteroids.remove(idx);
                return true;
            }
        }
        return false;
    }

    pub fn set_level(&mut self, level: u8) {
        if level > 1 {
            self.game_status = GameStatus::NewLevel;
        }
        self.level = level;
        self.asteroids.clear();
        let num_asteroids = 16 + 2 * level;
        let mut rng = rand::thread_rng();
        self.men_to_rescue = (level + 1) as u32;
        self.pods_remaining += (self.men_to_rescue as f32 * 1.6) as u32;
        for n in 0..num_asteroids {
            let max_speed = (4 + level) as i32;
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
        if self.pod_status == PodStatus::Dropping
            || self.pod_status == PodStatus::Landed
            || self.pod_status == PodStatus::ReadyForTakeOff
            || self.pod_status == PodStatus::Ascending
            || self.pod_status == PodStatus::AutoDock
            || self.game_status != GameStatus::Playing
        {
            return;
        };
        self.pod_status = PodStatus::Dropping;
        self.pod_pos_x = self.mothership_pos_x + 40.0;
        self.pod_pos_y = self.mothership_pos_y + 30.0;
    }

    pub fn launch_pod(&mut self) {
        if self.pod_status != PodStatus::ReadyForTakeOff
            || self.man_status == ManStatus::EnteringPod
        {
            return;
        }
        self.pod_status = PodStatus::Ascending;
    }

    pub fn pod_manoeuvre(&mut self, direction: PodMove) {
        if self.pod_status == PodStatus::Dropping || self.pod_status == PodStatus::Ascending {
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
