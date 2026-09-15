use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};

use crate::{
    event::InputEvent,
    music::{player::AudioPlayer, track::Track},
    renderer::{FONT_16, FONT_24, Image, Rect, Renderer, Scale, color::Color},
    ui::{
        PlayerAction,
        screen::{Screen, ScreenEvent, ScreenId},
        widget::{
            Button, ButtonStyle, Container, ContainerDirection, ContainerStyle, Dimension,
            ImageHandle, ImageWidget, Label, LabelHandle, LayoutParams, TextAlign, Widget,
        },
    },
};

pub struct Launcher<A>
where
    A: AudioPlayer,
{
    root: Container,
    player: Rc<RefCell<A>>,
    last_player_update: Instant,
    now_playing_title: LabelHandle,
    now_playing_artist: LabelHandle,
    cover: ImageHandle,
}

impl<A> Launcher<A>
where
    A: AudioPlayer,
{
    pub fn new(player: Rc<RefCell<A>>) -> Self {
        let button_style = ButtonStyle::default().with_radius(8);

        let mut root = Container::new(Rect::new(0, 0, 480, 320))
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
            .with_direction(ContainerDirection::Horizontal)
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
            .on_click(Box::new(|| ScreenEvent::Player(PlayerAction::Previous)));

        let pause_play = Button::new()
            .with_text(">")
            .with_style(button_style)
            .on_click(Box::new(|| ScreenEvent::Player(PlayerAction::PlayPause)));

        let next = Button::new()
            .with_text(">>")
            .with_style(button_style)
            .on_click(Box::new(|| ScreenEvent::Player(PlayerAction::Next)));

        let controll_panel = Container::new(Rect::default())
            .with_direction(ContainerDirection::Horizontal)
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

        let cover = ImageWidget::new(Image::default());
        let cover_handle = cover.handle();

        let now_playing = Container::new(Rect::default())
            .with_padding(12)
            .with_spacing(14)
            .with_direction(ContainerDirection::Horizontal)
            .with_style(
                ContainerStyle::default()
                    .with_background(Color::rgb(30, 30, 70))
                    .with_radius(8),
            )
            .with_child(
                cover,
                LayoutParams {
                    width: Dimension::Fixed(150),
                    height: Dimension::Fixed(150),
                },
            )
            .with_child(
                Container::new(Rect::default())
                    .with_direction(ContainerDirection::Vertical)
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
            .on_click(Box::new(|| ScreenEvent::Open(ScreenId::Music)));

        let settings = Button::new()
            .with_text("SETTINGS")
            .with_style(button_style)
            .on_click(Box::new(|| ScreenEvent::Open(ScreenId::Settings)));

        let row = Container::new(Rect::default())
            .with_direction(ContainerDirection::Horizontal)
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

        root = root
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
        Self {
            root,
            now_playing_title,
            now_playing_artist,
            last_player_update: Instant::now(),
            player,
            cover: cover_handle,
        }
    }

    pub fn set_current_track(&mut self, track: Option<Track>, library: Option<PathBuf>) {
        let mut title = self.now_playing_title.borrow_mut();
        let mut artist = self.now_playing_artist.borrow_mut();

        match (track, library) {
            (Some(track), Some(library)) => {
                *title = track.title.clone();
                *artist = track.artist.clone();

                let cover_path = library.parent().unwrap().join("cover.jpg");

                match Image::load(
                    cover_path,
                    Scale::Exact {
                        width: 150,
                        height: 150,
                    },
                ) {
                    Ok(cover) => {
                        self.cover.set(cover);
                    }

                    Err(error) => {
                        println!("Can't load cover for {}: {}", track.path.display(), error);
                    }
                }
            }

            _ => {
                *title = "Нет трека".to_string();
                *artist = String::new();
                self.cover.set(Image::default());
            }
        }
    }
}

impl<A> Screen for Launcher<A>
where
    A: AudioPlayer,
{
    fn update(&mut self, dt: Duration) {
        self.root.update(dt);
        if self.last_player_update.elapsed() >= Duration::from_millis(250) {
            self.last_player_update = Instant::now();
            let (track, path) = {
                let mut player = self.player.borrow_mut();
                if let Ok(Some(track)) = player.current_track() {
                    let path = player.current_song_path().ok();
                    (Some(track), path)
                } else {
                    (None, None)
                }
            };

            self.set_current_track(track, path);
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        self.root.render(renderer);
    }

    fn handle_input(&mut self, event: &InputEvent) -> ScreenEvent {
        self.root.handle_input(event)
    }
}
