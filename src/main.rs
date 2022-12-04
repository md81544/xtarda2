use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2i;
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};

mod game;

fn main() {
    let screen_width = VideoMode::desktop_mode().width;
    let screen_height = VideoMode::desktop_mode().height;
    let ratio: f32 = screen_width as f32 / screen_height as f32;

    let window_width = if screen_width >= 1920 {
        1920
    } else {
        screen_width
    };
    let window_height = (window_width as f32 / ratio) as u32;

    let mut window = RenderWindow::new(
        (window_width, window_height),
        "Xtarda Rescue",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i::new(50, 50));
    window.set_mouse_cursor_visible(false);
    window.set_key_repeat_enabled(false);
    let explosion_file = "res/explosion.wav";
    let explosion = sfml::audio::SoundBuffer::from_file(&explosion_file).unwrap();
    let mut explosion_sound = sfml::audio::Sound::with_buffer(&explosion);

    let mut game = game::Game::new(window_width, window_height);
    game.set_level(1);

    let mut moving_left = false;
    let mut moving_right = false;

    // Main Loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyReleased { code, .. } => match code {
                    Key::Escape => {
                        window.close();
                    }
                    Key::Q => {
                        window.close();
                    }
                    Key::Down => {
                        game.drop_pod();
                    }
                    Key::Space => {
                        game.drop_pod();
                    }
                    Key::Left => {
                        moving_left = false;
                    }
                    Key::Right => {
                        moving_right = false;
                    }
                    _ => {}
                },
                Event::KeyPressed { code, .. } => match code {
                    Key::Left => {
                        moving_left = true;
                    }
                    Key::Right => {
                        moving_right = true;
                    }
                    _ => {}
                },
                _ => {} // ignore other events
            }
        }
        if moving_right {
            game.pod_manoeuvre(game::PodMove::Right);
        }
        if moving_left {
            game.pod_manoeuvre(game::PodMove::Left);
        }
        window.clear(Color::BLACK);
        game.next_frame();
        game.draw_screen(&mut window);
        if game.is_pod_exploding() {
            if explosion_sound.status() != sfml::audio::SoundStatus::PLAYING {
                explosion_sound.play();
            }
        }
        window.display();
    }
}
