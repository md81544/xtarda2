use std::cmp::Ordering;

use rand::Rng;
use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::Vector2f;
use sfml::SfBox;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let mut dist = distance(0.0, 0.0, 1.0, 0.0);
        assert!(dist == 1.0);
        dist = distance(0.0, 0.0, 1.0, 1.0);
        assert!(dist > 1.414 && dist < 1.415);
    }

    #[test]
    fn collision_check_true() {
        let mut game = Game::new(1920, 1280, "res".to_string());
        game.pod_pos_x = 10.0;
        game.pod_pos_y = 10.0;

        let asteroid = Asteroid {
            y_pos: 0.0,
            x_pos: 0.0,
            speed: 0.0,
            r1: 30.0,
            r1_offset_x: 0.0,
            r1_offset_y: 0.0,
            r2: 30.0,
            r2_offset_x: 20.0,
            r2_offset_y: 0.0,
            r3: 10.0,
            r3_offset_x: 60.0,
            r3_offset_y: 0.0,
        };
        game.asteroids.push(asteroid);
        assert!(game.check_for_pod_collision() == CollisionType::Fatal);
        // Asteroid should have been destroyed
        assert!(game.asteroids.is_empty());
    }

    #[test]
    fn collision_check_false() {
        let mut game = Game::new(1920, 1280, "res".to_string());
        game.pod_pos_x = 10.0;
        game.pod_pos_y = 10.0;

        let asteroid = Asteroid {
            y_pos: 100.0,
            x_pos: 100.0,
            speed: 0.0,
            r1: 30.0,
            r1_offset_x: 0.0,
            r1_offset_y: 0.0,
            r2: 30.0,
            r2_offset_x: 20.0,
            r2_offset_y: 0.0,
            r3: 10.0,
            r3_offset_x: 60.0,
            r3_offset_y: 0.0,
        };
        game.asteroids.push(asteroid);
        assert!(game.check_for_pod_collision() == CollisionType::None);
        assert!(game.asteroids.len() > 0);
    }
}

struct Asteroid {
    // Note all values are size_multiplier adjusted
    y_pos: f32,
    x_pos: f32,
    speed: f32,
    r1: f32,
    r1_offset_x: f32,
    r1_offset_y: f32,
    r2: f32,
    r2_offset_x: f32,
    r2_offset_y: f32,
    r3: f32,
    r3_offset_x: f32,
    r3_offset_y: f32,
}

struct Star {
    y_pos: u32,
    x_pos: u32,
    radius: u8,
    luminosity: u8,
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
    Seatbelt,
    TakeOff,
    DropPod,
    Scrape,
    Bonus,
}

#[derive(Eq, PartialEq)]
pub enum GameStatus {
    Playing,
    SplashScreen,
    GameOver,
    NewLevel,
    Paused,
}

#[derive(Eq, PartialEq)]
enum CollisionType {
    None,
    NearMiss,
    Fatal,
}

pub struct Game {
    pub game_status: GameStatus,
    level: u8,
    window_width: u32,
    window_height: u32,
    size_multiplier: f32,
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
    pods_carried_over: u32,
    man_pos_x: f32,
    man_pos_y: f32,
    man_status: ManStatus,
    pub debugging_aids: bool,
    stars: Vec<Star>,
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    f32::sqrt((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0))
}

impl Game {
    pub fn new(window_width: u32, window_height: u32, resource_path: String) -> Game {
        let size_multiplier = window_width as f32 * 0.000_520_833_36;
        let pad_width = 250.0 * size_multiplier;
        let font = Font::from_file(&(resource_path + "/zx-spectrum.ttf")).unwrap();
        Game {
            game_status: GameStatus::SplashScreen,
            level: 1,
            window_width,
            window_height,
            size_multiplier,
            mothership_pos_x: 50.0,
            mothership_pos_y: 100.0 * size_multiplier,
            mothership_direction: 10,
            mothership_width: (80_f32 * size_multiplier) as u32,
            asteroids: Vec::new(),
            pod_pos_x: 0.0,
            pod_pos_y: 100.0,
            font,
            ground_height: 40.0 * size_multiplier,
            landing_pad_height: 20.0 * size_multiplier,
            landing_pad_width: pad_width,
            landing_pad_x: window_width as f32 / 2.0 - (pad_width / 2.0),
            pod_size: 20.0 * size_multiplier,
            pod_status: PodStatus::Inactive,
            pod_explosion_timer: 0,
            sounds_to_play: vec![],
            men_to_rescue: 5,
            pods_remaining: 0,
            pods_carried_over: 0,
            man_pos_x: window_width as f32 * 0.75,
            man_pos_y: window_height as f32 - 60.0 * size_multiplier,
            man_status: ManStatus::Inactive,
            debugging_aids: false,
            stars: Vec::new(),
        }
    }

    fn draw_mothership(&mut self, window: &mut RenderWindow) {
        let mut fin = RectangleShape::with_size(Vector2f::new(
            15.0 * self.size_multiplier,
            15.0 * self.size_multiplier,
        ));
        fin.set_fill_color(Color::rgb(0, 255, 0));
        fin.set_position(Vector2f::new(
            self.mothership_pos_x,
            self.mothership_pos_y + 15.0 * self.size_multiplier,
        ));
        window.draw(&fin);
        fin.set_position(Vector2f::new(
            self.mothership_pos_x + 65.0 * self.size_multiplier,
            self.mothership_pos_y + 15.0 * self.size_multiplier,
        ));
        window.draw(&fin);
        let mut body = RectangleShape::with_size(Vector2f::new(
            50.0 * self.size_multiplier,
            30.0 * self.size_multiplier,
        ));
        body.set_fill_color(Color::rgb(0, 255, 0));
        body.set_position(Vector2f::new(
            self.mothership_pos_x + 15.0 * self.size_multiplier,
            self.mothership_pos_y,
        ));
        window.draw(&body);
    }

    fn draw_man(&mut self, window: &mut RenderWindow) {
        if self.man_status == ManStatus::Inactive {
            return;
        }
        let mut man = RectangleShape::with_size(Vector2f::new(
            6.0 * self.size_multiplier,
            20.0 * self.size_multiplier,
        ));
        man.set_fill_color(Color::rgb(0, 255, 0));
        man.set_position(Vector2f::new(self.man_pos_x, self.man_pos_y));
        window.draw(&man);
    }

    fn draw_stars(&mut self, window: &mut RenderWindow) {
        for star in &self.stars {
            let mut circle = CircleShape::new(star.radius as f32, 4);
            circle.set_fill_color(Color::rgb(0, star.luminosity, 0));
            circle.set_position(Vector2f::new(star.x_pos as f32, star.y_pos as f32));
            window.draw(&circle);
        }
    }

    fn draw_ground(&mut self, window: &mut RenderWindow) {
        let ground_colour = 96;
        let hill_colour = 64;
        let mut hill1 = CircleShape::new(150.0 * self.size_multiplier, 3);
        hill1.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill1.set_position(Vector2f::new(
            0.0,
            self.window_height as f32 - 150.0 * self.size_multiplier,
        ));
        window.draw(&hill1);
        let mut hill2 = CircleShape::new(300.0 * self.size_multiplier, 3);
        hill2.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill2.set_position(Vector2f::new(
            -300.0 * self.size_multiplier,
            self.window_height as f32 - 300.0 * self.size_multiplier,
        ));
        window.draw(&hill2);
        let mut hill3 = CircleShape::new(240.0 * self.size_multiplier, 3);
        hill3.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill3.set_position(Vector2f::new(
            self.window_width as f32 - 400.0 * self.size_multiplier,
            self.window_height as f32 - 240.0 * self.size_multiplier,
        ));
        window.draw(&hill3);
        let mut hill4 = CircleShape::new(340.0 * self.size_multiplier, 3);
        hill4.set_fill_color(Color::rgb(0, hill_colour, 0));
        hill4.set_position(Vector2f::new(
            self.window_width as f32 - 370.0 * self.size_multiplier,
            self.window_height as f32 - 340.0 * self.size_multiplier,
        ));
        window.draw(&hill4);
        let mut ground =
            RectangleShape::with_size(Vector2f::new(self.window_width as f32, self.ground_height));
        ground.set_fill_color(Color::rgb(0, ground_colour, 0));
        ground.set_position(Vector2f::new(
            0.0,
            self.window_height as f32 - 40.0 * self.size_multiplier,
        ));
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
        let mut moonbase = CircleShape::new(100.0 * self.size_multiplier, 32);
        moonbase.set_fill_color(Color::rgb(0, 110, 0));
        moonbase.set_position(Vector2f::new(
            self.window_width as f32 * 0.75,
            self.window_height as f32 - 100.0 * self.size_multiplier - self.ground_height,
        ));
        window.draw(&moonbase);
    }

    fn draw_asteroids(&mut self, window: &mut RenderWindow) {
        for asteroid in &self.asteroids {
            let mut blob3 = CircleShape::new(asteroid.r3, 8);
            blob3.set_fill_color(Color::rgb(0, 80, 0));
            blob3.set_position(Vector2f::new(
                asteroid.x_pos + asteroid.r3_offset_x,
                asteroid.y_pos + asteroid.r3_offset_y,
            ));
            window.draw(&blob3);
            let mut blob2 = CircleShape::new(asteroid.r2, 8);
            blob2.set_fill_color(Color::rgb(0, 100, 0));
            blob2.set_position(Vector2f::new(
                asteroid.x_pos + asteroid.r2_offset_x,
                asteroid.y_pos + asteroid.r2_offset_y,
            ));
            window.draw(&blob2);
            let mut blob1 = CircleShape::new(asteroid.r1, 8);
            blob1.set_fill_color(Color::rgb(0, 120, 0));
            blob1.set_position(Vector2f::new(
                asteroid.x_pos + asteroid.r1_offset_x,
                asteroid.y_pos + asteroid.r1_offset_y,
            ));
            window.draw(&blob1);
        }
    }

    fn draw_pod(&mut self, window: &mut RenderWindow) {
        if self.pod_status == PodStatus::Exploding {
            let mut rng = rand::thread_rng();
            let radius = rng.gen_range(20.0 * self.size_multiplier..200.0 * self.size_multiplier);
            let mut explosion = CircleShape::new(radius, 32);
            let lum = rng.gen_range(200..255);
            explosion.set_fill_color(Color::rgb(0, lum, 0));
            explosion.set_position(Vector2f::new(
                self.pod_pos_x - radius + self.pod_size / 2.0,
                self.pod_pos_y - radius + self.pod_size / 2.0,
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

    fn draw_status_bar(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!(
                "Level: {}  Terrans to Rescue: {}  Pods Left: {}",
                self.level, self.men_to_rescue, self.pods_remaining
            ),
            &self.font,
            (self.window_width as f32 * 0.015625) as u32,
        );
        text.set_position(Vector2f::new(
            200.0 * self.size_multiplier,
            20.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
    }

    fn draw_splash_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &"Xtarda Rescue!".to_string(),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            200.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_message("(c) 1982 Sonic Software", window);
        self.draw_press_enter(window);
    }

    fn draw_new_level_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &format!("Level {}", self.level),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            200.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        match 1.cmp(&self.pods_carried_over) {
            Ordering::Greater => {}
            Ordering::Equal => {
                self.draw_message("1 pod carried over", window);
            }
            Ordering::Less => {
                self.draw_message(
                    &format!("{} pods carried over", self.pods_carried_over),
                    window,
                );
            }
        }
        self.draw_press_enter(window);
    }

    fn draw_pause_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            "Paused",
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            200.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_press_enter(window);
    }

    fn draw_game_over_screen(&mut self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &"Game Over".to_string(),
            &self.font,
            (self.window_width as f32 * 0.05) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            200.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 200, 0));
        window.draw(&text);
        self.draw_restart_yn(window);
    }

    fn draw_press_enter(&self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &"Press ENTER to continue".to_string(),
            &self.font,
            (self.window_width as f32 * 0.02) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            600.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 150, 0));
        window.draw(&text);
    }

    fn draw_message(&self, msg: &str, window: &mut RenderWindow) {
        let mut text = Text::new(msg, &self.font, (self.window_width as f32 * 0.02) as u32);
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            350.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 120, 0));
        window.draw(&text);
    }

    fn draw_restart_yn(&self, window: &mut RenderWindow) {
        let mut text = Text::new(
            &"Restart? Y/N".to_string(),
            &self.font,
            (self.window_width as f32 * 0.02) as u32,
        );
        text.set_position(Vector2f::new(
            150.0 * self.size_multiplier,
            500.0 * self.size_multiplier,
        ));
        text.set_fill_color(Color::rgb(0, 150, 0));
        window.draw(&text);
    }

    pub fn draw_screen(&mut self, window: &mut RenderWindow) {
        match self.game_status {
            GameStatus::Playing => {
                self.draw_stars(window);
                self.draw_mothership(window);
                self.draw_moonbase(window);
                self.draw_landing_pad(window);
                self.draw_man(window);
                self.draw_ground(window);
                self.draw_asteroids(window);
                self.draw_status_bar(window);
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
            GameStatus::Paused => {
                self.draw_pause_screen(window);
            }
        }
    }

    pub fn restart(&mut self) {
        self.new_level(1);
        self.game_status = GameStatus::SplashScreen;
        self.pod_status = PodStatus::Inactive;
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
        self.mothership_pos_x += self.mothership_direction as f32 * self.size_multiplier;
        for asteroid in &mut self.asteroids {
            asteroid.x_pos += asteroid.speed;
            if asteroid.speed > 0.0 && asteroid.x_pos > self.window_width as f32 {
                asteroid.x_pos = -150.0 * self.size_multiplier;
            }
            if asteroid.speed < 0.0 && asteroid.x_pos < -150.0 * self.size_multiplier {
                asteroid.x_pos = self.window_width as f32;
            }
        }
        if self.pod_status == PodStatus::Dropping && !self.check_for_pod_landing() {
            match self.check_for_pod_collision() {
                CollisionType::Fatal => {
                    self.explode_pod();
                }
                CollisionType::NearMiss => {
                    self.sounds_to_play.push(Sounds::Scrape);
                }
                _ => {}
            }
            self.pod_pos_y += 5.0 * self.size_multiplier;
        }
        if self.pod_status == PodStatus::Ascending && !self.check_for_pod_docking() {
            match self.check_for_pod_collision() {
                CollisionType::Fatal => {
                    self.explode_pod();
                }
                CollisionType::NearMiss => {
                    self.sounds_to_play.push(Sounds::Scrape);
                }
                _ => {}
            }
            self.pod_pos_y -= 5.0 * self.size_multiplier;
        }
        if self.pod_status == PodStatus::AutoDock {
            if self.mothership_pos_x < self.pod_pos_x {
                self.pod_pos_x -= 20.0 * self.size_multiplier;
            } else {
                self.pod_pos_x += 20.0 * self.size_multiplier;
            }
            self.check_for_pod_docking();
        }
        if self.man_status == ManStatus::EnteringPod {
            if self.man_pos_x > self.pod_pos_x + 15.0 {
                self.man_pos_x -= 10.0 * self.size_multiplier;
            } else {
                self.man_status = ManStatus::Inactive;
                self.sounds_to_play.push(Sounds::Seatbelt);
                self.man_pos_x = self.window_width as f32 * 0.75;
                self.man_pos_y = self.window_height as f32 - 60.0 * self.size_multiplier;
            }
        }
    }

    fn explode_pod(&mut self) {
        self.pod_status = PodStatus::Exploding;
        self.sounds_to_play.push(Sounds::Explosion);
        if !self.debugging_aids {
            self.pods_remaining -= 1;
            if self.pods_remaining == 0 {
                self.game_over();
            }
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
                if self.pod_status != PodStatus::AutoDock {
                    // bonus for not needing autodock
                    self.pods_remaining += 1;
                    self.sounds_to_play.push(Sounds::Bonus);
                }
                self.sounds_to_play.push(Sounds::Docked);
                self.pod_status = PodStatus::Inactive;
                self.men_to_rescue -= 1;
                if self.men_to_rescue == 0 {
                    self.new_level(self.level + 1);
                }
                return true;
            } else {
                self.pod_status = PodStatus::AutoDock;
            }
        }
        false
    }

    fn check_for_pod_collision(&mut self) -> CollisionType {
        let pod_centre_x = self.pod_pos_x + (self.pod_size / 2.0);
        let pod_centre_y = self.pod_pos_y + (self.pod_size / 2.0);
        for (idx, asteroid) in self.asteroids.iter().enumerate() {
            let blob1_centre_x = asteroid.x_pos + asteroid.r1_offset_x + asteroid.r1;
            let blob1_centre_y = asteroid.y_pos + asteroid.r1_offset_y + asteroid.r1;
            let mut dist = distance(pod_centre_x, pod_centre_y, blob1_centre_x, blob1_centre_y);
            if dist <= asteroid.r1 {
                self.asteroids.remove(idx);
                return CollisionType::Fatal;
            }
            if dist <= asteroid.r1 + self.pod_size / 2.0 {
                return CollisionType::NearMiss;
            }
            let blob2_centre_x = asteroid.x_pos + asteroid.r2_offset_x + asteroid.r2;
            let blob2_centre_y = asteroid.y_pos + asteroid.r2_offset_y + asteroid.r2;
            dist = distance(pod_centre_x, pod_centre_y, blob2_centre_x, blob2_centre_y);
            if dist <= asteroid.r2 {
                self.asteroids.remove(idx);
                return CollisionType::Fatal;
            }
            if dist <= asteroid.r2 + self.pod_size / 2.0 {
                return CollisionType::NearMiss;
            }
            let blob3_centre_x = asteroid.x_pos + asteroid.r3_offset_x + asteroid.r3;
            let blob3_centre_y = asteroid.y_pos + asteroid.r3_offset_y + asteroid.r3;
            dist = distance(pod_centre_x, pod_centre_y, blob3_centre_x, blob3_centre_y);
            if dist <= asteroid.r3 {
                self.asteroids.remove(idx);
                return CollisionType::Fatal;
            }
            if dist <= asteroid.r3 + self.pod_size / 2.0 {
                return CollisionType::NearMiss;
            }
        }
        CollisionType::None
    }

    pub fn new_level(&mut self, level: u8) {
        if level > 1 {
            self.game_status = GameStatus::NewLevel;
        }
        self.level = level;
        self.asteroids.clear();
        let num_asteroids = 16 + 2 * level;
        let mut rng = rand::thread_rng();
        self.men_to_rescue = (level + 1) as u32;
        self.pods_carried_over = self.pods_remaining;
        self.pods_remaining += 1 + (self.men_to_rescue as f32 * 0.25) as u32;
        let asteroid_min_y = self.window_height as f32 * 0.144;
        let asteroid_max_y = self.window_height as f32 * 0.7;
        let asteroid_vertical_spacing = (asteroid_max_y - asteroid_min_y) / num_asteroids as f32;
        for n in 0..num_asteroids {
            let max_speed = 3.0 + (level as f32) / 2.0;
            let mut speed = rng.gen_range(-max_speed..max_speed);
            if speed > -0.25 && speed < 0.25 {
                speed = 0.25 * speed.signum();
            }
            speed *= self.size_multiplier;
            let asteroid = Asteroid {
                y_pos: asteroid_min_y + asteroid_vertical_spacing * n as f32,
                x_pos: rng.gen_range(50.0..self.window_width as f32 - 50.0),
                speed,
                r1: rng.gen_range(20.0 * self.size_multiplier..40.0 * self.size_multiplier),
                r1_offset_x: 0.0 * self.size_multiplier, // yes I know
                r1_offset_y: rng.gen_range(0.0..30.00 * self.size_multiplier),
                r2: rng.gen_range(30.0 * self.size_multiplier..50.0 * self.size_multiplier),
                r2_offset_x: 20.0 * self.size_multiplier,
                r2_offset_y: rng.gen_range(0.0..10.00 * self.size_multiplier),
                r3: rng.gen_range(20.0 * self.size_multiplier..40.0 * self.size_multiplier),
                r3_offset_x: 60.0 * self.size_multiplier,
                r3_offset_y: rng.gen_range(0.0..30.00 * self.size_multiplier),
            };
            self.asteroids.push(asteroid);
        }
        if self.stars.is_empty() {
            for _ in 0..320 {
                self.stars.push(Star {
                    y_pos: rng
                        .gen_range((self.window_height as f32 * 0.075) as u32..self.window_height),
                    x_pos: rng.gen_range(0..self.window_width),
                    radius: rng.gen_range(
                        (2.0 * self.size_multiplier) as u8..(5.0 * self.size_multiplier) as u8,
                    ),
                    luminosity: rng.gen_range(64..128),
                });
            }
        }
    }

    pub fn drop_pod(&mut self) {
        if self.game_status != GameStatus::Playing {
            return;
        }
        if self.pod_status != PodStatus::Inactive && self.pod_status != PodStatus::Exploding {
            return;
        };
        self.pod_status = PodStatus::Dropping;
        self.sounds_to_play.push(Sounds::DropPod);
        self.pod_pos_x = self.mothership_pos_x + 40.0;
        self.pod_pos_y = self.mothership_pos_y + 30.0;
    }

    pub fn launch_pod(&mut self) {
        if self.game_status != GameStatus::Playing {
            return;
        }
        if self.pod_status != PodStatus::ReadyForTakeOff
            || self.man_status == ManStatus::EnteringPod
        {
            return;
        }
        self.pod_status = PodStatus::Ascending;
        self.sounds_to_play.push(Sounds::TakeOff);
    }

    pub fn pod_manoeuvre(&mut self, direction: PodMove) {
        if self.pod_status == PodStatus::Dropping || self.pod_status == PodStatus::Ascending {
            match direction {
                PodMove::Left => {
                    self.pod_pos_x -= 4.0 * self.size_multiplier;
                }
                PodMove::Right => {
                    self.pod_pos_x += 4.0 * self.size_multiplier;
                }
            }
        }
    }
}
