mod app;
mod event;
mod platform;
mod renderer;
mod ui;

use app::App;
#[cfg(feature = "desktop")]
use platform::desktop::Desktop;

#[cfg(feature = "raspberry")]
use platform::raspberry::Raspberry;

fn main() {
    #[cfg(feature = "raspberry")]
    let platform = Raspberry::new().unwrap();
    #[cfg(feature = "desktop")]
    let platform = Desktop::new().unwrap();

    let mut app = App::new(platform);

    let result = app.run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
