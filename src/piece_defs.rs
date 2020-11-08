use super::conf::*;
use super::pieces::Piece;

/* piece constructors */
// [r, g, b, a]
// [shape]
// (origin x, origin y)

pub fn i() -> Piece {
    Piece::new(
        COL_I,
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        (1.5, 0.0),
        true,
        4,
        1,
    )
}

pub fn o() -> Piece {
    Piece::new(
        COL_O,
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        (0.5, 0.5),
        false,
        2,
        2,
    )
}

pub fn j() -> Piece {
    Piece::new(
        COL_J,
        [(1, 0), (1, 1), (1, 2), (0, 2)],
        (1.0, 1.0),
        true,
        2,
        3,
    )
}

pub fn l() -> Piece {
    Piece::new(
        COL_L,
        [(0, 0), (0, 1), (0, 2), (1, 2)],
        (0.0, 1.0),
        true,
        2,
        3,
    )
}

pub fn s() -> Piece {
    Piece::new(
        COL_S,
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        (1.0, 1.0),
        true,
        3,
        2,
    )
}

pub fn z() -> Piece {
    Piece::new(
        COL_Z,
        [(1, 1), (2, 1), (0, 0), (1, 0)],
        (1.0, 1.0),
        true,
        3,
        2,
    )
}

pub fn t() -> Piece {
    Piece::new(
        COL_T,
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        (1.0, 1.0),
        true,
        3,
        2,
    )
}
