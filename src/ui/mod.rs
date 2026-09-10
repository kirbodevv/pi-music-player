mod button;
pub mod input;

use crate::{renderer::Renderer, ui::input::InputEvent};

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

            music_button: Button::new(40, 60, 180, 80, 0x07E0, 0x03E0),

            files_button: Button::new(260, 60, 180, 80, 0x001F, 0x0010),

            settings_button: Button::new(40, 180, 180, 80, 0xF800, 0x7800),

            about_button: Button::new(260, 180, 180, 80, 0xFFE0, 0x7BE0),
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
        renderer.clear(0x0000);

        self.music_button.render(renderer);
        self.files_button.render(renderer);
        self.settings_button.render(renderer);
        self.about_button.render(renderer);
    }

    fn render_music(&self, renderer: &mut Renderer) {
        renderer.clear(0x0000);
    }

    fn render_settings(&self, renderer: &mut Renderer) {
        renderer.clear(0x0000);
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

    pub fn handle_input(&mut self, event: InputEvent) {
        match event {
            InputEvent::TouchDown { x, y } => {
                self.touch_down(x, y);
            }

            InputEvent::TouchUp { x, y } => {
                self.touch_up(x, y);
            }

            InputEvent::TouchMove { x, y } => {
                self.touch_move(x, y);
            }
        }
    }
}
