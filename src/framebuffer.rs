use std::fs::OpenOptions;
use std::io;
use std::os::fd::AsRawFd;
use std::ptr;

use crate::renderer::{HEIGHT, WIDTH};

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

    pub fn present(&mut self, pixels: &[u16]) {
        unsafe {
            ptr::copy_nonoverlapping(pixels.as_ptr(), self.ptr, WIDTH * HEIGHT);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, FB_SIZE);
        }
    }
}
