pub const MAXX: i8 = 10;
pub const MAXY: i8 = 24;
pub const BOARDY: i8 = 22;
pub const CELLSIZE: f64 = 20.0;
pub const RATE: i8 = 25; // lower rate = faster drop

pub type Color = [f32; 4];
pub type Point = (i8, i8);
pub type FPoint = (f64, f64);

pub enum States {
    Stop,
    End,
    Nothing,
}
