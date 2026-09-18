include!(concat!(env!("OUT_DIR"), "/generated_icons.rs"));

use super::Renderer;
use crate::renderer::{Color, Rect};

impl Renderer {
    pub fn draw_icon(&mut self, icon: Icon, rect: Rect, color: Color) {
        let icon = icon_data(icon);

        assert_eq!(icon.width, icon.height);

        if rect.width != icon.width as usize || rect.height != icon.height as usize {
            todo!("icon scaling");
        }

        for y in 0..icon.height as usize {
            for x in 0..icon.width as usize {
                let index = y * icon.width as usize + x;

                let byte = icon.data[index / 2];

                let alpha = if index % 2 == 0 {
                    byte >> 4
                } else {
                    byte & 0x0F
                };

                if alpha == 0 {
                    continue;
                }

                let alpha = alpha * 17;

                let pixel = color.with_alpha(alpha);

                self.pixel(rect.x + x, rect.y + y, pixel);
            }
        }
    }
}
