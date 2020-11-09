use rand::seq::SliceRandom;

use super::conf::BOARDY;
use super::conf::*;
use super::piece_defs::*;
use super::pieces::Piece;

pub const MAXY: i8 = BOARDY + 2;
pub const WELLWIDTH: f64 = MAXX as f64 * CELLSIZE;
pub const RWIDTH: f64 = CELLSIZE * 7.0;
pub const NWIDTH: f64 = RWIDTH - CELLSIZE * 2.0;
pub const NHEIGHT: f64 = CELLSIZE * 4.0;
pub const WINSIZE: (f64, f64) = (WELLWIDTH + RWIDTH, BOARDY as f64 * CELLSIZE);
pub const TOP_PAD: f64 = (MAXY - BOARDY) as f64 * CELLSIZE;

pub const PIECES: [fn() -> Piece; 7] = [i, o, j, l, s, z, t];

pub type Color = [f32; 4];
pub type Point = (i8, i8);
pub type FPoint = (f64, f64);

pub fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()()
}

pub enum States {
    Stop,
    End,
    Pause,
    DrawStatic,
    Nothing,
}
