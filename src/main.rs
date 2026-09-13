pub mod app;
pub mod event;
pub mod music;
pub mod platform;
pub mod renderer;
pub mod ui;

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

    let app = App::new(platform);

    let Ok(mut app) = app else {
        eprintln!("Failed to create app: {}", app.err().unwrap());
        return;
    };

    let result = app.run();

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
