use crate::music::service::MusicService;

pub struct Context {
    pub music: MusicService,
}

impl Context {
    pub fn new(music: MusicService) -> Self {
        Self { music }
    }

    pub fn tick(&mut self) {
        self.music.poll();
    }
}
