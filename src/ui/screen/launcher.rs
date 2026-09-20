use std::path::PathBuf;
use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    music::track::Track,
    renderer::{FONT_16, FONT_24, Icon, Image, Rect, Renderer, color::Color},
    ui::{
        screen::{Screen, ScreenId, Transition},
        widget::{
            Button, ButtonStyle, Container, ContainerDirection, ContainerStyle, Dimension,
            ImageHandle, ImageWidget, Label, LabelHandle, LayoutParams, TextAlign, Widget,
        },
    },
};

pub struct Launcher {
    root: Container,
    now_playing_title: LabelHandle,
    now_playing_artist: LabelHandle,
    cover: ImageHandle,
    displayed_track_path: Option<PathBuf>,
}

impl Launcher {
    pub fn new() -> Self {
        let button_style = ButtonStyle::default().with_radius(8);

        let mut root = Container::new(Rect::new(0, 0, 480, 320))
            .with_padding(16)
            .with_spacing(10);

        /*
         * HEADER
         */

        const SCREEN_BACKGROUND: Color = Color::rgb(15, 15, 20);

        let title = Label::new("Main Menu")
            .with_font(&FONT_24)
            .with_color(Color::rgb(240, 240, 245))
            .with_background(SCREEN_BACKGROUND);

        let status = Label::new("12:48   •   78%")
            .with_font(&FONT_16)
            .with_color(Color::rgb(150, 155, 170))
            .with_text_align(TextAlign::Right)
            .with_background(SCREEN_BACKGROUND);

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
            .with_icon(Icon::SkipBack, 18)
            .with_style(button_style)
            .on_click(Box::new(|ctx: &mut Context| {
                ctx.music.previous();
                Transition::None
            }));

        let pause_play = Button::new()
            .with_icon(Icon::Play, 18)
            .with_style(button_style)
            .on_click(Box::new(|ctx: &mut Context| {
                ctx.music.play_pause();
                Transition::None
            }));

        let next = Button::new()
            .with_icon(Icon::SkipForward, 18)
            .with_style(button_style)
            .on_click(Box::new(|ctx: &mut Context| {
                ctx.music.next();
                Transition::None
            }));

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

        const NOW_PLAYING_BACKGROUND: Color = Color::rgb(30, 30, 70);

        let track_title = Label::new("Нет трека")
            .with_font(&FONT_24)
            .with_color(Color::WHITE)
            .with_background(NOW_PLAYING_BACKGROUND);
        let now_playing_title = track_title.handle();

        let track_artist = Label::new("")
            .with_font(&FONT_16)
            .with_color(Color::rgb(130, 135, 150))
            .with_background(NOW_PLAYING_BACKGROUND);
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
                            .with_color(Color::rgb(130, 135, 150))
                            .with_background(NOW_PLAYING_BACKGROUND),
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
            .on_click(Box::new(|_ctx: &mut Context| {
                Transition::Open(ScreenId::Music)
            }));

        let settings = Button::new()
            .with_text("SETTINGS")
            .with_style(button_style)
            .on_click(Box::new(|_ctx: &mut Context| {
                Transition::Open(ScreenId::Settings)
            }));

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
            cover: cover_handle,
            displayed_track_path: None,
        }
    }

    fn set_track_labels(&mut self, track: Option<&Track>) {
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
    }
}

impl Screen for Launcher {
    fn update(&mut self, ctx: &mut Context, dt: Duration) {
        self.root.update(dt);

        let path = ctx.music.current_song_path().cloned();

        if path != self.displayed_track_path {
            self.displayed_track_path = path.clone();

            let track = ctx.music.current_track().cloned();
            self.set_track_labels(track.as_ref());

            let cover = match &path {
                Some(path) => ctx
                    .artwork
                    .cover_for(path, crate::artwork::DEFAULT_COVER_SIZE),
                None => Image::default(),
            };
            self.cover.set(cover);
        }
    }

    fn render(&mut self, renderer: &mut Renderer) {
        self.root.render(renderer);
    }

    fn handle_input(&mut self, event: &InputEvent, ctx: &mut Context) -> Transition {
        self.root.handle_input(event, ctx)
    }

    fn mark_dirty(&mut self) {
        self.root.mark_dirty();
    }

    fn clear_dirty(&mut self) {
        self.root.clear_dirty();
    }
}
