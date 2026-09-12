#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    PointDown { x: i32, y: i32 },
    PointUp { x: i32, y: i32 },
    PointMove { x: i32, y: i32 },
}
