use std::fs::OpenOptions;
use std::io;
use std::os::fd::AsRawFd;
use std::ptr;

use crate::renderer::{HEIGHT, Rect, WIDTH};

const FB_SIZE: usize = WIDTH * HEIGHT * 2;

pub struct Framebuffer {
    _file: std::fs::File,
    ptr: *mut u16,
}

impl Framebuffer {
    pub fn open() -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open("/dev/fb1")?;

        let ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                FB_SIZE,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            _file: file,
            ptr: ptr as *mut u16,
        })
    }

    pub fn present(&mut self, pixels: &[u16], dirty: Rect) -> io::Result<()> {
        if dirty.is_empty() {
            return Ok(());
        }

        let x_end = (dirty.x + dirty.width).min(WIDTH);
        let y_end = (dirty.y + dirty.height).min(HEIGHT);

        let width = x_end - dirty.x;

        for y in dirty.y..y_end {
            let offset = y * WIDTH + dirty.x;

            unsafe {
                ptr::copy_nonoverlapping(pixels.as_ptr().add(offset), self.ptr.add(offset), width);
            }
        }

        Ok(())
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, FB_SIZE);
        }
    }
}
