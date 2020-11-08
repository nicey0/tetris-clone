use super::colpoint::ColPoint;
use super::pieces::*;
use super::util::*;

pub struct Shadow {
    pub shape: [Point; 4],
}

impl Shadow {
    pub fn new(piece: &Piece) -> Shadow {
        Shadow {
            shape: *piece.get_shape(),
        }
    }
    pub fn down(&mut self, pieces: &Vec<ColPoint>) -> bool {
        // move in y axis
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after movement
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0, self.shape[i].1 + 1);
            if shape[i].1 >= MAXY
                || pieces.contains(&ColPoint {
                    point: shape[i],
                    color: [0.0; 4],
                })
            {
                return false;
            }
        }
        // set shape and origin to moved values & return Nothing
        self.shape = shape;
        true
    }

    pub fn put_down(&mut self, pieces: &Vec<ColPoint>) {
        loop {
            if !self.down(pieces) {
                break;
            }
        }
    }
}
