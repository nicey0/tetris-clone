use super::util::Color;

// board
pub const MAXX: i8 = 10;
pub const BOARDY: i8 = 20;
pub const CELLSIZE: f64 = 30.0;

pub const WELL: Color = [0.05, 0.05, 0.05, 1.0];
pub const RIGHT: Color = [0.25, 0.25, 0.25, 1.0];
pub const NEXT: Color = [0.1, 0.1, 0.1, 1.0];

// clears for next level
pub const CLEARS: u8 = 6;

// rate lost per level
pub const RL: u16 = 5;

// FPS
pub const FPS: u64 = 35;
// gravity
pub const INIRATE: u16 = 45; // lower rate = faster drop

// piece colors
pub const COL_I: Color = [0.0, 1.0, 1.0, 1.0];
pub const COL_O: Color = [1.0, 1.0, 0.0, 1.0];
pub const COL_J: Color = [0.0, 0.0, 1.0, 1.0];
pub const COL_L: Color = [1.0, 0.5, 0.0, 1.0];
pub const COL_S: Color = [0.0, 1.0, 0.0, 1.0];
pub const COL_Z: Color = [1.0, 0.0, 0.0, 1.0];
pub const COL_T: Color = [0.5, 0.0, 0.8, 1.0];
pub const SHDOW: Color = [0.3, 0.3, 0.3, 1.0];
