use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;

#[derive(Debug)]
pub struct State {
    pub rate: u16,
    pub mrate: u16,
    pub cl: u8,
    pub p: Piece,
    pub shadow: Shadow,
    pub next: Piece,
    pub pieces: Vec<ColPoint>,
    pub score: u32,
}

// Driver code
impl State {
    pub fn new() -> Self {
        let p = random_piece();
        Self {
            //  gravity
            mrate: INIRATE,
            cl: 0,
            rate: 0,
            // Vector holding still pieces
            pieces: Vec::new(),
            p,
            shadow: Shadow::new(&p),
            next: random_piece(),
            score: 0,
        }
    }
}
