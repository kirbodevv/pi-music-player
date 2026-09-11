mod button;

use crate::{
    platform::Event,
    renderer::{Renderer, color::Color},
};

use button::Button;

pub struct Ui {
    screen: Screen,

    music_button: Button,
    files_button: Button,
    settings_button: Button,
    about_button: Button,
}

enum Screen {
    Launcher,
    Music,
    Settings,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            screen: Screen::Launcher,

            music_button: Button::new(40, 60, 180, 80, Color::RED, Color::MAGENTA),

            files_button: Button::new(260, 60, 180, 80, Color::BLACK, Color::GRAY),

            settings_button: Button::new(40, 180, 180, 80, Color::BLUE, Color::CYAN),

            about_button: Button::new(260, 180, 180, 80, Color::YELLOW, Color::WHITE),
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        match self.screen {
            Screen::Launcher => self.render_launcher(renderer),
            Screen::Music => self.render_music(renderer),
            Screen::Settings => self.render_settings(renderer),
        }
    }

    fn render_launcher(&self, renderer: &mut Renderer) {
        renderer.clear(Color::BLACK);

        self.music_button.render(renderer);
        self.files_button.render(renderer);
        self.settings_button.render(renderer);
        self.about_button.render(renderer);
    }

    fn render_music(&self, renderer: &mut Renderer) {
        renderer.clear(Color::BLACK);
    }

    fn render_settings(&self, renderer: &mut Renderer) {
        renderer.clear(Color::BLACK);
    }
    pub fn touch_down(&mut self, x: usize, y: usize) {
        match self.screen {
            Screen::Launcher => {
                if self.music_button.contains(x, y) {
                    self.music_button.set_pressed(true);
                }

                if self.files_button.contains(x, y) {
                    self.files_button.set_pressed(true);
                }

                if self.settings_button.contains(x, y) {
                    self.settings_button.set_pressed(true);
                }

                if self.about_button.contains(x, y) {
                    self.about_button.set_pressed(true);
                }
            }

            Screen::Music => {}
            Screen::Settings => {}
        }
    }

    pub fn touch_up(&mut self, x: usize, y: usize) {
        match self.screen {
            Screen::Launcher => {
                if self.music_button.contains(x, y) {
                    println!("Music clicked");
                }

                if self.files_button.contains(x, y) {
                    println!("Files clicked");
                }

                if self.settings_button.contains(x, y) {
                    println!("Settings clicked");
                }

                if self.about_button.contains(x, y) {
                    println!("About clicked");
                }

                self.music_button.set_pressed(false);
                self.files_button.set_pressed(false);
                self.settings_button.set_pressed(false);
                self.about_button.set_pressed(false);
            }

            Screen::Music => {}
            Screen::Settings => {}
        }
    }

    fn touch_move(&mut self, _x: usize, _y: usize) {
        // Пока ничего.
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            Event::TouchDown { x, y } => {
                self.touch_down(x as usize, y as usize);
            }

            Event::TouchUp { x, y } => {
                self.touch_up(x as usize, y as usize);
            }

            Event::TouchMove { x, y } => {
                self.touch_move(x as usize, y as usize);
            }
            _ => {}
        }
    }
}
