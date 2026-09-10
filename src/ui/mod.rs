mod button;

use crate::renderer::Renderer;

use button::Button;

pub struct Ui {
    screen: Screen,
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
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        match self.screen {
            Screen::Launcher => self.render_launcher(renderer),
            Screen::Music => self.render_music(renderer),
            Screen::Settings => self.render_settings(renderer),
        }
    }

    fn render_launcher(&mut self, renderer: &mut Renderer) {
        renderer.clear(0x0000);

        let music = Button::new(40, 60, 180, 80, 0x07E0);
        let files = Button::new(260, 60, 180, 80, 0x001F);
        let settings = Button::new(40, 180, 180, 80, 0xF800);
        let about = Button::new(260, 180, 180, 80, 0xFFE0);

        music.render(renderer);
        files.render(renderer);
        settings.render(renderer);
        about.render(renderer);
    }

    fn render_music(&mut self, renderer: &mut Renderer) {
        renderer.clear(0x0000);
    }

    fn render_settings(&mut self, renderer: &mut Renderer) {
        renderer.clear(0x0000);
    }
}
