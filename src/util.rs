use super::conf::BOARDY;

pub const MAXY: i8 = BOARDY + 2;

pub type Color = [f32; 4];
pub type Point = (i8, i8);
pub type FPoint = (f64, f64);

pub enum States {
    Stop,
    End,
    Nothing,
}
