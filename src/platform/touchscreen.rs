use std::path::Path;
use std::{io, os::fd::AsRawFd};

use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode};

use crate::platform::Event;

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

        let fd = device.as_raw_fd();

        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL);

            if flags == -1 {
                return Err(io::Error::last_os_error());
            }

            if libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(Self {
            device,

            touching: false,

            last_raw_x: 0,
            last_raw_y: 0,

            last_x: 0,
            last_y: 0,
        })
    }
    pub fn poll(&mut self) -> io::Result<Vec<Event>> {
        let mut events = Vec::new();

        let was_touching = self.touching;

        let mut x_changed = false;
        let mut y_changed = false;

        let fetched = match self.device.fetch_events() {
            Ok(events) => events,

            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Ok(events);
            }

            Err(e) => return Err(e),
        };

        for event in fetched {
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
            events.push(Event::TouchDown {
                x: self.last_x as i32,
                y: self.last_y as i32,
            });
        }

        if was_touching && self.touching && (x_changed || y_changed) {
            events.push(Event::TouchMove {
                x: self.last_x as i32,
                y: self.last_y as i32,
            });
        }

        if was_touching && !self.touching {
            events.push(Event::TouchUp {
                x: self.last_x as i32,
                y: self.last_y as i32,
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
