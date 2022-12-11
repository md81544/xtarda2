use std::path::Path;
use std::process::exit;

use sfml::audio::Music;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2i;
use sfml::window::{ContextSettings, Event, Key, Style, VideoMode};

mod game;

fn main() {
    let screen_width = VideoMode::desktop_mode().width;
    let screen_height = VideoMode::desktop_mode().height;
    let ratio: f32 = screen_width as f32 / screen_height as f32;

    let mut resource_path = "res".to_string();
    let mut count = 0;
    loop {
        if Path::new(&(resource_path.to_string() + "/zx-spectrum.ttf")).exists() {
            break;
        }
        count += 1;
        if count > 3 {
            panic!("Could not find font file")
        };
        resource_path = "../".to_string() + &resource_path;
    }

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
    window.set_vertical_sync_enabled(true);
    window.set_position(Vector2i::new(50, 50));
    window.set_mouse_cursor_visible(false);
    window.set_key_repeat_enabled(false);
    let explosion =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/explosion.wav")).unwrap();
    let mut explosion_sound = sfml::audio::Sound::with_buffer(&explosion);
    let landed =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/success.wav")).unwrap();
    let mut landed_sound = sfml::audio::Sound::with_buffer(&landed);
    let docked =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/docked.wav")).unwrap();
    let mut docked_sound = sfml::audio::Sound::with_buffer(&docked);
    let seatbelt =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/seatbelt.wav")).unwrap();
    let mut seatbelt_sound = sfml::audio::Sound::with_buffer(&seatbelt);
    let take_off =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/take_off.wav")).unwrap();
    let mut take_off_sound = sfml::audio::Sound::with_buffer(&take_off);
    let drop_pod =
        sfml::audio::SoundBuffer::from_file(&(resource_path.clone() + "/drop_pod.wav")).unwrap();
    let mut drop_pod_sound = sfml::audio::Sound::with_buffer(&drop_pod);
    let music_file = resource_path.clone() + "/background.wav";
    let mut music = Music::from_file(&music_file).unwrap();
    music.set_looping(true);
    music.play();

    let mut game = game::Game::new(window_width, window_height, resource_path);
    game.new_level(1);
    if game.debugging_aids {
        window.set_framerate_limit(10);
    } else {
        window.set_framerate_limit(60);
    }

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
                    Key::Y => {
                        if game.game_status == game::GameStatus::GameOver {
                            game.restart();
                        }
                    }
                    Key::N => {
                        if game.game_status == game::GameStatus::GameOver {
                            exit(0);
                        }
                    }
                    Key::Down => {
                        game.drop_pod();
                    }
                    Key::Up => {
                        game.launch_pod();
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
                    Key::Enter => {
                        if game.game_status != game::GameStatus::GameOver {
                            game.game_status = game::GameStatus::Playing;
                        }
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
        for sound in &game.sounds_to_play {
            match sound {
                game::Sounds::Docked => {
                    docked_sound.play();
                }
                game::Sounds::Explosion => {
                    explosion_sound.play();
                }
                game::Sounds::Landed => {
                    landed_sound.play();
                }
                game::Sounds::Seatbelt => {
                    seatbelt_sound.play();
                }
                game::Sounds::TakeOff => {
                    take_off_sound.play();
                }
                game::Sounds::DropPod => {
                    drop_pod_sound.play();
                }
            }
        }
        game.sounds_to_play.clear();
        window.display();
    }
}
