use std::path::PathBuf;
use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{FONT_16, FONT_24, Image, Rect, Renderer, color::Color},
    ui::{
        prelude::*,
        widget::Dimension::{Auto, Fill, Fixed},
    },
};

pub struct Launcher {
    root: Container,
    cover: Handle<ImageState>,
    displayed_track_path: Option<PathBuf>,
}

impl Launcher {
    pub fn new() -> Self {
        let button_style = ButtonStyle::default().with_radius(8).with_icon_padding(18);

        const SCREEN_BACKGROUND: Color = Color::rgb(15, 15, 20);

        let title = label()
            .text("Main Menu")
            .font(&FONT_24)
            .color(Color::rgb(240, 240, 245))
            .background(SCREEN_BACKGROUND);

        let status = label()
            .text("12:48   •   78%")
            .font(&FONT_16)
            .color(Color::rgb(150, 155, 170))
            .text_align(TextAlign::Right)
            .background(SCREEN_BACKGROUND);

        let header = container()
            .direction(ContainerDirection::Horizontal)
            .spacing(8)
            .with_child(title, layout(Fill, Fixed(32)))
            .with_child(status, layout(Auto, Fixed(32)));

        const NOW_PLAYING_BACKGROUND: Color = Color::rgb(30, 30, 70);

        let cover = image();
        let cover_handle = cover.handle();

        let now_playing = container()
            .padding(12)
            .spacing(14)
            .direction(ContainerDirection::Horizontal)
            .style(
                ContainerStyle::default()
                    .with_background(Color::rgb(30, 30, 70))
                    .with_radius(8),
            )
            .with_child(cover, layout(Fixed(150), Fixed(150)))
            .with_child(
                container()
                    .direction(ContainerDirection::Vertical)
                    .spacing(4)
                    .with_child(
                        label()
                            .text("СЕЙЧАС ИГРАЕТ")
                            .font(&FONT_16)
                            .color(Color::rgb(130, 135, 150))
                            .background(NOW_PLAYING_BACKGROUND),
                        layout(Fill, Fixed(20)),
                    ),
                layout(Fill, Fill),
            );

        let music = button()
            .text("MUSIC")
            .style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Open(ScreenId::Music));

        let settings = button()
            .text("SETTINGS")
            .style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Open(ScreenId::Settings));

        let row = container()
            .direction(ContainerDirection::Horizontal)
            .spacing(10)
            .with_child(music, layout(Fill, Fill))
            .with_child(settings, layout(Fill, Fill));

        /*
         * ROOT
         */

        let root = container()
            .rect(Rect::new(0, 0, 480, 320))
            .padding(16)
            .spacing(10)
            .with_child(header, layout(Fill, Fixed(32)))
            .with_child(now_playing, layout(Fill, Fixed(185)))
            .with_child(row, layout(Fill, Fill));

        Self {
            root,
            cover: cover_handle,
            displayed_track_path: None,
        }
    }
}

impl Screen for Launcher {
    fn update(&mut self, ctx: &mut Context, dt: Duration) {
        self.root.update(dt);

        let path = ctx.music.current_song_path().cloned();

        if path != self.displayed_track_path {
            self.displayed_track_path = path.clone();

            let image = match &path {
                Some(path) => {
                    ctx.artwork
                        .cover_for(path, crate::artwork::DEFAULT_COVER_SIZE)
                        .image
                }
                None => Image::default(),
            };
            self.cover.modify(|state| state.image = image);
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
