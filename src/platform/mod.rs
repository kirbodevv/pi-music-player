use crate::platform;

#[cfg(feature = "desktop")]
mod desktop;

#[cfg(feature = "raspberry")]
mod raspberry;

pub struct Platform {
    backend: Backend,
}

enum Backend {
    #[cfg(feature = "desktop")]
    Desktop(desktop::DesktopPlatform),

    #[cfg(feature = "raspberry")]
    Raspberry(raspberry::RaspberryPlatform),
}

impl Platform {
    pub fn new() -> Self {
        Self {
            backend: Backend::new(),
        }
    }

    pub fn is_running(&mut self) -> bool {
        match &mut self.backend {
            #[cfg(feature = "desktop")]
            Backend::Desktop(platform) => platform.is_running(),

            #[cfg(feature = "raspberry")]
            Backend::Raspberry(platform) => platform.is_running(),

            _ => false,
        }
    }

    pub fn present(&mut self, pixels: &[u16]) {
        match &mut self.backend {
            #[cfg(feature = "desktop")]
            Backend::Desktop(platform) => platform.present(pixels),

            #[cfg(feature = "raspberry")]
            Backend::Raspberry(platform) => platform.present(pixels),

            _ => {}
        }
    }
}

impl Backend {
    fn new() -> Self {
        #[cfg(feature = "desktop")]
        {
            return Backend::Desktop(desktop::DesktopPlatform::new());
        }

        #[cfg(feature = "raspberry")]
        {
            return Backend::Raspberry(raspberry::RaspberryPlatform::new());
        }

        panic!("Can't run on this platform");
    }
}
