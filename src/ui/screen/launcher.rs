use std::path::PathBuf;
use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{FONT_16, FONT_24, Image, Rect, Renderer, color::Color},
    ui::prelude::*,
};

pub struct Launcher {
    root: Container,
    cover: Handle<ImageState>,
    displayed_track_path: Option<PathBuf>,
}

impl Launcher {
    pub fn new() -> Self {
        let button_style = ButtonStyle::default().with_radius(8).with_icon_padding(18);

        let mut root = Container::new(Rect::new(0, 0, 480, 320))
            .padding(16)
            .spacing(10);

        /*
         * HEADER
         */

        const SCREEN_BACKGROUND: Color = Color::rgb(15, 15, 20);

        let title = Label::new("Main Menu")
            .font(&FONT_24)
            .color(Color::rgb(240, 240, 245))
            .background(SCREEN_BACKGROUND);

        let status = Label::new("12:48   •   78%")
            .font(&FONT_16)
            .color(Color::rgb(150, 155, 170))
            .text_align(TextAlign::Right)
            .background(SCREEN_BACKGROUND);

        let header = Container::new(Rect::default())
            .direction(ContainerDirection::Horizontal)
            .spacing(8)
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

        const NOW_PLAYING_BACKGROUND: Color = Color::rgb(30, 30, 70);

        let cover = ImageWidget::new(Image::default());
        let cover_handle = cover.handle();

        let now_playing = Container::new(Rect::default())
            .padding(12)
            .spacing(14)
            .direction(ContainerDirection::Horizontal)
            .style(
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
                    .direction(ContainerDirection::Vertical)
                    .spacing(4)
                    .with_child(
                        Label::new("СЕЙЧАС ИГРАЕТ")
                            .font(&FONT_16)
                            .color(Color::rgb(130, 135, 150))
                            .background(NOW_PLAYING_BACKGROUND),
                        LayoutParams {
                            width: Dimension::Fill,
                            height: Dimension::Fixed(20),
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
            .text("MUSIC")
            .style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Open(ScreenId::Music));

        let settings = Button::new()
            .text("SETTINGS")
            .style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Open(ScreenId::Settings));

        let row = Container::new(Rect::default())
            .direction(ContainerDirection::Horizontal)
            .spacing(10)
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
