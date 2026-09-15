use std::collections::HashMap;

use crate::ui::screen::{Screen, ScreenId, launcher::Launcher};

/// Constructs every screen the UI knows about. This is the single place
/// that needs to change when a new screen is added, keeping `Ui` itself
/// focused on navigation/dispatch rather than on how each screen is built.
pub fn build_screens() -> HashMap<ScreenId, Box<dyn Screen>> {
    let mut screens = HashMap::<ScreenId, Box<dyn Screen>>::new();

    screens.insert(ScreenId::Launcher, Box::new(Launcher::new()));

    screens
}
