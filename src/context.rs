use crate::artwork::ArtworkService;
use crate::music::service::MusicService;

pub struct Context {
    pub music: MusicService,
    pub artwork: ArtworkService,
}

impl Context {
    pub fn new(music: MusicService) -> Self {
        Self {
            music,
            artwork: ArtworkService::new(),
        }
    }

    pub fn tick(&mut self) {
        self.music.poll();
    }
}
