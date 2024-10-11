pub trait Object {
    fn draw(&self);
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
