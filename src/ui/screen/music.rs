use std::path::PathBuf;
use std::time::Duration;

use crate::{
    artwork::{Artwork, ArtworkTheme},
    context::Context,
    event::InputEvent,
    music::{player::PlayerState, track::Track},
    renderer::{FONT_16, FONT_24, Icon, Image, Rect, Renderer, color::Color},
    ui::{
        prelude::*,
        widget::Dimension::{Fill, Fixed},
    },
};

const COVER_SIZE: usize = 262;

pub struct MusicScreen {
    root: Container,
    progress_bar: Handle<ProgressBarState>,
    now_playing_title: Handle<LabelState>,
    now_playing_artist: Handle<LabelState>,
    play_pause_button: Handle<ButtonState>,
    background: Handle<ContainerState>,
    cover: Handle<ImageState>,
    displayed_track_path: Option<PathBuf>,
}

impl MusicScreen {
    pub fn new() -> Self {
        let button_style = ButtonStyle::default().with_radius(8).with_icon_padding(4);

        let back = button()
            .icon(Icon::ArrowLeft)
            .style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Back);

        let heading = label()
            .text("Now Playing")
            .font(&FONT_16)
            .color(Color::rgb(240, 240, 245))
            .text_align(TextAlign::Center)
            .vertical_align(VerticalAlign::Center)
            .background(Color::rgb(15, 15, 20));

        let header = container()
            .direction(ContainerDirection::Horizontal)
            .spacing(8)
            .with_child(back, layout(Fixed(32), Fixed(32)))
            .with_child(heading, layout(Fill, Fixed(32)))
            .with_child(container(), layout(Fixed(32), Fixed(32)));

        /*
         * NOW PLAYING
         */

        let track_title = label()
            .text("Нет трека")
            .font(&FONT_24)
            .color(Color::WHITE)
            .background(Color::rgb(30, 30, 70));
        let now_playing_title = track_title.handle();

        let track_artist = label()
            .font(&FONT_16)
            .color(Color::rgb(130, 135, 150))
            .background(Color::rgb(30, 30, 70));
        let now_playing_artist = track_artist.handle();

        let cover = image().radius(8);
        let cover_handle = cover.handle();

        let progress_bar = progress_bar().min(0.).max(1.);
        let progress_bar_handle = progress_bar.handle();

        let cotroll_buttons_style = button_style.with_icon_padding(12);

        let previous = button()
            .icon(Icon::SkipBack)
            .style(cotroll_buttons_style)
            .on_click(|ctx: &mut Context| {
                ctx.music.previous();
                Transition::None
            });

        let pause_play = button()
            .icon(Icon::Play)
            .style(cotroll_buttons_style)
            .on_click(|ctx: &mut Context| {
                ctx.music.play_pause();
                Transition::None
            });

        let play_pause_button = pause_play.handle();

        let next = button()
            .icon(Icon::SkipForward)
            .style(cotroll_buttons_style)
            .on_click(|ctx: &mut Context| {
                ctx.music.next();
                Transition::None
            });

        let controll_buttons_layout = layout(Fixed(48), Fixed(48));
        let controll_panel = container()
            .direction(ContainerDirection::Horizontal)
            .spacing(8)
            .with_child(previous, controll_buttons_layout)
            .with_child(pause_play, controll_buttons_layout)
            .with_child(next, controll_buttons_layout);

        let track_info = container()
            .direction(ContainerDirection::Vertical)
            .spacing(2)
            .with_child(track_artist, layout(Fill, Fixed(20)))
            .with_child(track_title, layout(Fill, Fixed(40)));

        let info = container()
            .direction(ContainerDirection::Vertical)
            .spacing(8)
            .with_child(track_info, layout(Fill, Fixed(62)))
            .with_child(container(), layout(Fill, Fixed(98)))
            .with_child(
                container()
                    .direction(ContainerDirection::Horizontal)
                    .with_child(container(), layout(Fixed(14), Fill))
                    .with_child(controll_panel, layout(Fill, Fixed(60))),
                layout(Fill, Fill),
            )
            .with_child(progress_bar, layout(Fill, Fixed(10)))
            .with_child(container(), layout(Fill, Fixed(6)));

        let background = container()
            .padding(8)
            .spacing(8)
            .direction(ContainerDirection::Horizontal)
            .style(ContainerStyle::default().with_radius(8))
            .with_child(cover, layout(Fixed(COVER_SIZE), Fixed(COVER_SIZE)))
            .with_child(info, layout(Fill, Fill));

        let background_handle = background.handle();

        let root = container()
            .rect(Rect::new(0, 0, 480, 320))
            .padding(4)
            .spacing(4)
            .with_child(header, layout(Fill, Fixed(30)))
            .with_child(background, layout(Fill, Fill));

        Self {
            root,
            progress_bar: progress_bar_handle,
            cover: cover_handle,
            now_playing_artist,
            now_playing_title,
            play_pause_button,
            background: background_handle,
            displayed_track_path: None,
        }
    }

    fn set_progress_bar(&mut self, position: Duration, duration: Duration) {
        let value = position.div_duration_f64(duration);
        self.progress_bar.modify(|state| state.set_value(value));
    }

    fn set_track_labels(&mut self, track: Option<&Track>) {
        match track {
            Some(track) => {
                self.now_playing_title.modify(|s| {
                    s.text = track.title.clone();
                });
                self.now_playing_artist.modify(|s| {
                    s.text = track.artist.clone();
                });
            }

            None => {
                self.now_playing_title.modify(|s| {
                    s.text = "Нет трека".to_string();
                });
                self.now_playing_artist.modify(|s| {
                    s.text = String::new();
                });
            }
        }
    }

    fn set_play_pause_button_icon(&self, state: PlayerState) {
        let icon = match state {
            PlayerState::Playing => Icon::Pause,
            PlayerState::Paused => Icon::Play,
            PlayerState::Stopped => Icon::Play,
        };

        self.play_pause_button
            .modify(|state| state.icon = Some(icon));
    }
}

impl Screen for MusicScreen {
    fn update(&mut self, ctx: &mut Context, dt: Duration) {
        self.root.update(dt);

        let path = ctx.music.current_song_path().cloned();

        if path != self.displayed_track_path {
            self.displayed_track_path = path.clone();

            let artwork = match &path {
                Some(path) => ctx.artwork.cover_for(path, COVER_SIZE as u32),
                None => Artwork {
                    image: Image::default(),
                    theme: ArtworkTheme::default(),
                },
            };

            let track = ctx.music.current_track().cloned();
            self.set_track_labels(track.as_ref());
            self.cover.modify(|state| state.image = artwork.image);
            self.background
                .modify(|state| state.style.background = Some(artwork.theme.dark));
            self.now_playing_artist.modify(|state| {
                state.background = Some(artwork.theme.dark);
                state.color = artwork.theme.light;
            });
            self.now_playing_title.modify(|state| {
                state.background = Some(artwork.theme.dark);
                state.color = artwork.theme.light;
            });
        }
        let position = ctx.music.current_position();
        let duration = ctx.music.current_duration();

        self.set_play_pause_button_icon(ctx.music.state());
        self.set_progress_bar(position, duration);
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
