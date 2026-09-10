use std::io;
use std::path::Path;

use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode};

use crate::ui::input::InputEvent;

const WIDTH: usize = 480;
const HEIGHT: usize = 320;

const TOUCH_MIN: f32 = 0.0;
const TOUCH_MAX: f32 = 4095.0;

pub struct Touchscreen {
    device: Device,

    touching: bool,

    last_raw_x: i32,
    last_raw_y: i32,

    last_x: usize,
    last_y: usize,
}

impl Touchscreen {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let device = Device::open(path)?;

        println!("Touchscreen: {}", device.name().unwrap_or("unknown"));

        Ok(Self {
            device,

            touching: false,

            last_raw_x: 0,
            last_raw_y: 0,

            last_x: 0,
            last_y: 0,
        })
    }

    pub fn poll(&mut self) -> io::Result<Vec<InputEvent>> {
        let mut events = Vec::new();

        let was_touching = self.touching;

        let mut x_changed = false;
        let mut y_changed = false;

        for event in self.device.fetch_events()? {
            match event.destructure() {
                EventSummary::AbsoluteAxis(_, code, value) => match code {
                    AbsoluteAxisCode::ABS_X => {
                        self.last_raw_x = value;
                        x_changed = true;
                    }

                    AbsoluteAxisCode::ABS_Y => {
                        self.last_raw_y = value;
                        y_changed = true;
                    }

                    _ => {}
                },

                EventSummary::Key(_, code, value) => {
                    if code != KeyCode::BTN_TOUCH {
                        continue;
                    }

                    match value {
                        1 => self.touching = true,
                        0 => self.touching = false,
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        if x_changed || y_changed {
            self.update_position();
        }

        if !was_touching && self.touching {
            events.push(InputEvent::TouchDown {
                x: self.last_x,
                y: self.last_y,
            });
        }

        if was_touching && self.touching && (x_changed || y_changed) {
            events.push(InputEvent::TouchMove {
                x: self.last_x,
                y: self.last_y,
            });
        }

        if was_touching && !self.touching {
            events.push(InputEvent::TouchUp {
                x: self.last_x,
                y: self.last_y,
            });
        }

        Ok(events)
    }
    fn update_position(&mut self) {
        self.last_x = raw_to_x(self.last_raw_x);
        self.last_y = raw_to_y(self.last_raw_y);
    }
}

fn normalize(value: i32) -> f32 {
    ((value as f32 - TOUCH_MIN) / (TOUCH_MAX - TOUCH_MIN)).clamp(0.0, 1.0)
}

fn raw_to_x(raw_y: i32) -> usize {
    let ty = normalize(raw_y);

    let x = ((1.0 - ty) * WIDTH as f32) as usize;

    x.min(WIDTH - 1)
}

fn raw_to_y(raw_x: i32) -> usize {
    let tx = normalize(raw_x);

    let y = (tx * HEIGHT as f32) as usize;

    y.min(HEIGHT - 1)
}
