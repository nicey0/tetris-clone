use super::util::Color;

pub const MAXX: i8 = 10;
pub const BOARDY: i8 = 20;
pub const CELLSIZE: f64 = 20.0;
pub const RATE: i8 = 25; // lower rate = faster drop
pub const SHADOWCOLOR: Color = [0.3, 0.3, 0.3, 1.0];

// piece colors
pub const COL_I: Color = [0.0, 1.0, 1.0, 1.0];
pub const COL_O: Color = [1.0, 1.0, 0.0, 1.0];
pub const COL_J: Color = [0.0, 0.0, 1.0, 1.0];
pub const COL_L: Color = [1.0, 0.5, 0.0, 1.0];
pub const COL_S: Color = [0.0, 1.0, 0.0, 1.0];
pub const COL_Z: Color = [1.0, 0.0, 0.0, 1.0];
pub const COL_T: Color = [0.5, 0.0, 0.8, 1.0];
