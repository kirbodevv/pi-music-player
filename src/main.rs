pub mod app;
pub mod event;
pub mod music;
pub mod platform;
pub mod renderer;
pub mod ui;

use app::App;

fn main() {
    #[cfg(feature = "raspberry")]
    #[allow(unused_variables)]
    let (platform, stream) = {
        use platform::raspberry::Raspberry;
        use std::os::unix::net::UnixStream;
        (
            Raspberry::new().unwrap(),
            UnixStream::connect("/run/mpd/socket"),
        )
    };

    #[cfg(feature = "desktop")]
    let (platform, stream) = {
        use platform::desktop::Desktop;
        use std::net::TcpStream;
        (
            Desktop::new().unwrap(),
            TcpStream::connect("127.0.0.1:6600"),
        )
    };

    let Ok(stream) = stream else {
        eprintln!("Failed to connect to MPD: {}", stream.err().unwrap());
        return;
    };

    let app = App::new(platform, stream);

    let Ok(mut app) = app else {
        eprintln!("Failed to create app: {}", app.err().unwrap());
        return;
    };

    let result = app.run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
