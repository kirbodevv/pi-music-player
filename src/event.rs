#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    TouchDown { x: i32, y: i32 },
    TouchUp { x: i32, y: i32 },
    TouchMove { x: i32, y: i32 },

    Quit,
}
