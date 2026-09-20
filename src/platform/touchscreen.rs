use std::path::Path;
use std::{io, os::fd::AsRawFd};

use evdev::{AbsoluteAxisCode, Device, EventSummary, KeyCode};

use crate::event::InputEvent;

const WIDTH: usize = 480;
const HEIGHT: usize = 320;

const RAW_X_TOP: i32 = 3800;
const RAW_X_BOTTOM: i32 = 200;

const RAW_Y_LEFT: i32 = 200;
const RAW_Y_RIGHT: i32 = 3800;

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
    pub fn poll(&mut self) -> io::Result<Vec<InputEvent>> {
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
            events.push(InputEvent::PointDown {
                x: self.last_x as i32,
                y: self.last_y as i32,
            });
        }

        if was_touching && self.touching && (x_changed || y_changed) {
            events.push(InputEvent::PointMove {
                x: self.last_x as i32,
                y: self.last_y as i32,
            });
        }

        if was_touching && !self.touching {
            events.push(InputEvent::PointUp {
                x: self.last_x as i32,
                y: self.last_y as i32,
            });
        }

        Ok(events)
    }

    fn update_position(&mut self) {
        self.last_x = raw_to_x(self.last_raw_y);
        self.last_y = raw_to_y(self.last_raw_x);
    }
}

fn raw_to_x(raw_y: i32) -> usize {
    let t = (raw_y - RAW_Y_LEFT) as f32 / (RAW_Y_RIGHT - RAW_Y_LEFT) as f32;

    (t.clamp(0.0, 1.0) * (WIDTH - 1) as f32).round() as usize
}

fn raw_to_y(raw_x: i32) -> usize {
    let t = (RAW_X_TOP - raw_x) as f32 / (RAW_X_TOP - RAW_X_BOTTOM) as f32;

    (t.clamp(0.0, 1.0) * (HEIGHT - 1) as f32).round() as usize
}
