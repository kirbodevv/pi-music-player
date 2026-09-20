use std::path::PathBuf;
use std::time::Duration;

use crate::{
    context::Context,
    event::InputEvent,
    renderer::{FONT_16, Icon, Image, Rect, Renderer, color::Color},
    ui::prelude::*,
};

const COVER_SIZE: usize = 262;

pub struct MusicScreen {
    root: Container,
    progress_bar: ProgressBarHandle,
    cover: Handle<ImageState>,
    displayed_track_path: Option<PathBuf>,
}

impl MusicScreen {
    pub fn new() -> Self {
        let button_style = ButtonStyle::default().with_radius(8).with_icon_padding(4);

        /*
         * HEADER
         */

        let back = Button::new()
            .with_icon(Icon::ArrowLeft)
            .with_style(button_style)
            .on_click(|_ctx: &mut Context| Transition::Back);

        let heading = Label::new("Now Playing")
            .with_font(&FONT_16)
            .with_color(Color::rgb(240, 240, 245))
            .with_text_align(TextAlign::Center)
            .with_vertical_align(VerticalAlign::Center)
            .with_background(Color::rgb(15, 15, 20));

        let header = Container::default()
            .with_direction(ContainerDirection::Horizontal)
            .with_spacing(8)
            .with_child(
                back,
                LayoutParams {
                    width: Dimension::Fixed(32),
                    height: Dimension::Fixed(32),
                },
            )
            .with_child(
                heading,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(32),
                },
            )
            .with_child(
                Container::default(),
                LayoutParams {
                    width: Dimension::Fixed(32),
                    height: Dimension::Fixed(32),
                },
            );

        /*
         * NOW PLAYING
         */

        let cover = ImageWidget::new(Image::default());
        let cover_handle = cover.handle();

        let progress_bar = ProgressBar::new().with_min_value(0.).with_max_value(1.);
        let progress_bar_handle = progress_bar.handle();

        progress_bar_handle.borrow_mut().set_value(50.);

        let now_playing = Container::new(Rect::default())
            .with_padding(8)
            .with_spacing(20)
            .with_direction(ContainerDirection::Horizontal)
            .with_style(
                ContainerStyle::default()
                    .with_background(Color::rgb(30, 30, 70))
                    .with_radius(8),
            )
            .with_child(
                cover,
                LayoutParams {
                    width: Dimension::Fixed(COVER_SIZE),
                    height: Dimension::Fixed(COVER_SIZE),
                },
            )
            .with_child(
                progress_bar,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(10),
                },
            );

        /*
         * ROOT
         */

        let root = Container::new(Rect::new(0, 0, 480, 320))
            .with_padding(4)
            .with_spacing(4)
            .with_child(
                header,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fixed(30),
                },
            )
            .with_child(
                now_playing,
                LayoutParams {
                    width: Dimension::Fill,
                    height: Dimension::Fill,
                },
            );

        Self {
            root,
            progress_bar: progress_bar_handle,
            cover: cover_handle,
            displayed_track_path: None,
        }
    }

    fn set_progress_bar(&mut self, position: Duration, duration: Duration) {
        let mut progress_bar = self.progress_bar.borrow_mut();
        let value = position.as_secs_f64() / duration.as_secs_f64();
        progress_bar.set_value(value);
    }
}

impl Screen for MusicScreen {
    fn update(&mut self, ctx: &mut Context, dt: Duration) {
        self.root.update(dt);

        let path = ctx.music.current_song_path().cloned();

        if path != self.displayed_track_path {
            self.displayed_track_path = path.clone();

            let cover = match &path {
                Some(path) => ctx.artwork.cover_for(path, COVER_SIZE as u32),
                None => Image::default(),
            };
            self.cover.modify(|state| state.image = cover);
        }
        let position = ctx.music.current_position();
        let duration = ctx.music.current_duration();

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
