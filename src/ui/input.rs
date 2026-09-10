#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    TouchDown { x: usize, y: usize },
    TouchMove { x: usize, y: usize },
    TouchUp { x: usize, y: usize },
}
