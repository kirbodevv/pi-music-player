pub mod button;
pub mod container;
pub mod image;
pub mod label;
pub mod layout;
pub mod style;
pub mod widget;

use crate::{
    event::InputEvent,
    music::track::Track,
    renderer::{FONT_16, FONT_20, FONT_24, FONT_32, Image, Rect, Renderer, Scale, color::Color},
    ui::{
        container::{Container, Direction},
        image::ImageWidget,
        label::{Label, LabelHandle, TextAlign},
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

    now_playing_title: LabelHandle,
    now_playing_artist: LabelHandle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiEvent {
    None,
    Open(Screen),
    Back,

    Player(PlayerAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    Previous,
    PlayPause,
    Next,
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

        let button_style = ButtonStyle::default().with_radius(8);

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

        let previous = Button::new()
            .with_text("<<")
            .with_style(button_style)
            .on_click(Box::new(|| UiEvent::Player(PlayerAction::Previous)));

        let pause_play = Button::new()
            .with_text(">")
            .with_style(button_style)
            .on_click(Box::new(|| UiEvent::Player(PlayerAction::PlayPause)));

        let next = Button::new()
            .with_text(">>")
            .with_style(button_style)
            .on_click(Box::new(|| UiEvent::Player(PlayerAction::Next)));

        let controll_panel = Container::new(Rect::default())
            .with_direction(Direction::Horizontal)
            .with_spacing(8)
            .with_child(
                previous,
                LayoutParams {
                    width: Dimension::Fixed(60),
                    height: Dimension::Fixed(60),
                },
            )
            .with_child(
                pause_play,
                LayoutParams {
                    width: Dimension::Fixed(60),
                    height: Dimension::Fixed(60),
                },
            )
            .with_child(
                next,
                LayoutParams {
                    width: Dimension::Fixed(60),
                    height: Dimension::Fixed(60),
                },
            );

        let track_title = Label::new("Нет трека")
            .with_font(&FONT_24)
            .with_color(Color::WHITE);
        let now_playing_title = track_title.handle();

        let track_artist = Label::new("")
            .with_font(&FONT_16)
            .with_color(Color::rgb(130, 135, 150));
        let now_playing_artist = track_artist.handle();

        let now_playing = Container::new(Rect::default())
            .with_padding(12)
            .with_spacing(14)
            .with_direction(Direction::Horizontal)
            .with_style(
                ContainerStyle::default()
                    .with_background(Color::rgb(30, 30, 70))
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
                    .with_spacing(4)
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
                        track_title,
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(32),
                        },
                    )
                    .with_child(
                        track_artist,
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(22),
                        },
                    )
                    .with_child(
                        controll_panel,
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fill,
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
            now_playing_title,
            now_playing_artist,
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

    pub fn handle_input(&mut self, event: InputEvent) -> UiEvent {
        let ui_event = match self.screen {
            Screen::Launcher => self.launcher.handle_input(&event),
            Screen::Music => self.music.handle_input(&event),
            Screen::Settings => self.settings.handle_input(&event),
        };

        match &ui_event {
            UiEvent::None | UiEvent::Player(_) => {}

            UiEvent::Open(screen) => {
                self.change_screen(screen.clone());
            }

            UiEvent::Back => {
                self.change_screen(Screen::Launcher);
            }
        }

        ui_event
    }

    pub fn set_current_track(&mut self, track: Option<&Track>) {
        let mut title = self.now_playing_title.borrow_mut();
        let mut artist = self.now_playing_artist.borrow_mut();

        match track {
            Some(track) => {
                *title = track.title.clone();
                *artist = track.artist.clone();
            }

            None => {
                *title = "Нет трека".to_string();
                *artist = String::new();
            }
        }

        self.need_to_clear = true;
    }
}
