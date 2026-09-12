pub mod button;
pub mod container;
pub mod image;
pub mod label;
pub mod layout;
pub mod style;
pub mod widget;

use crate::{
    event::InputEvent,
    renderer::{FONT_16, FONT_20, FONT_24, FONT_32, Image, Rect, Renderer, Scale, color::Color},
    ui::{
        container::{Container, Direction},
        image::ImageWidget,
        label::{Label, TextAlign},
        layout::{Dimension, LayoutParams},
        style::{ButtonStyle, ContainerStyle},
        widget::Widget,
    },
};

use button::Button;

pub struct Ui {
    screen: Screen,

    launcher: Container,
    music: Container,
    settings: Container,

    need_to_clear: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    None,
    Open(Screen),
    Back,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Launcher,
    Music,
    Settings,
}

impl Ui {
    pub fn new() -> Self {
        const SCREEN_RECT: Rect = Rect {
            x: 0,
            y: 0,
            width: 480,
            height: 320,
        };

        let cover = Image::load("cover.jpg", Scale::FixedWidth(150)).unwrap_or(Image::default());

        let mut launcher = Container::new(Rect::new(0, 0, 480, 320))
            .with_padding(16)
            .with_spacing(10);

        /*
         * HEADER
         */

        let title = Label::new("Main Menu")
            .with_font(&FONT_24)
            .with_color(Color::rgb(240, 240, 245));

        let status = Label::new("12:48   •   78%")
            .with_font(&FONT_16)
            .with_color(Color::rgb(150, 155, 170))
            .with_text_align(TextAlign::Right);

        let header = Container::new(Rect::default())
            .with_direction(Direction::Horizontal)
            .with_spacing(8)
            .with_child(
                title,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(32),
                },
            )
            .with_child(
                status,
                LayoutParams {
                    width: Dimension::Auto,
                    height: Dimension::Fixed(32),
                },
            );

        /*
         * NOW PLAYING
         */

        let now_playing = Container::new(Rect::default())
            .with_padding(12)
            .with_spacing(14)
            .with_direction(Direction::Horizontal)
            .with_style(
                ContainerStyle::default()
                    .with_background(Color::rgb(100, 135, 150))
                    .with_radius(8),
            )
            .with_child(
                ImageWidget::new(cover),
                LayoutParams {
                    width: Dimension::Fixed(150),
                    height: Dimension::Fixed(150),
                },
            )
            .with_child(
                Container::new(Rect::default())
                    .with_direction(Direction::Vertical)
                    .with_spacing(2)
                    .with_child(
                        Label::new("СЕЙЧАС ИГРАЕТ")
                            .with_font(&FONT_16)
                            .with_color(Color::rgb(130, 135, 150)),
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(20),
                        },
                    )
                    .with_child(
                        Label::new("Не перегори")
                            .with_font(&FONT_24)
                            .with_color(Color::WHITE),
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(32),
                        },
                    )
                    .with_child(
                        Label::new("STERVELL")
                            .with_font(&FONT_16)
                            .with_color(Color::rgb(170, 175, 190)),
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(22),
                        },
                    ),
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            );

        /*
         * APPLICATIONS
         */

        let button_style = ButtonStyle::default().with_radius(8);

        let music = Button::new()
            .with_text("MUSIC")
            .with_style(button_style)
            .on_click(Box::new(|| UiEvent::Open(Screen::Music)));

        let settings = Button::new()
            .with_text("SETTINGS")
            .with_style(button_style)
            .on_click(Box::new(|| UiEvent::Open(Screen::Settings)));

        let row = Container::new(Rect::default())
            .with_direction(Direction::Horizontal)
            .with_spacing(10)
            .with_child(
                music,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            )
            .with_child(
                settings,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            );

        /*
         * ROOT
         */

        launcher = launcher
            .with_child(
                header,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(32),
                },
            )
            .with_child(
                now_playing,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(185),
                },
            )
            .with_child(
                row,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            );

        let music = Container::new(SCREEN_RECT);
        let settings = Container::new(SCREEN_RECT);

        Self {
            screen: Screen::Launcher,
            launcher,
            music,
            settings,
            need_to_clear: true,
        }
    }

    fn change_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.need_to_clear = true;
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        if self.need_to_clear {
            renderer.clear(Color::rgb(15, 15, 20));
            self.need_to_clear = false;
        }

        match self.screen {
            Screen::Launcher => self.launcher.render(renderer),
            Screen::Music => self.music.render(renderer),
            Screen::Settings => self.settings.render(renderer),
        }
    }

    pub fn handle_input(&mut self, event: InputEvent) {
        let ui_event = match self.screen {
            Screen::Launcher => self.launcher.handle_input(&event),
            Screen::Music => self.music.handle_input(&event),
            Screen::Settings => self.settings.handle_input(&event),
        };

        match ui_event {
            UiEvent::None => {}

            UiEvent::Open(screen) => {
                self.change_screen(screen);
            }

            UiEvent::Back => {
                self.change_screen(Screen::Launcher);
            }
        }
    }
}
