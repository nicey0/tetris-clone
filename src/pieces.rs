const ROT: usize = 4;

pub type Rotations = [Vec<(usize, usize)>; ROT];

#[derive(Clone)]
pub struct Piece {
    color: [f32; 4],
    rots: Rotations,
    idx: usize,
}

impl Piece {
    pub fn new(color: [f32; 4], rots: Rotations) -> Self {
        Piece {color, rots, idx: 0}
    }

    pub fn rotate(&mut self, dir: bool) {
        self.idx = if dir { // clockwise
            if self.idx < ROT-1 {
                self.idx + 1
            } else {
                0
            }
        } else { // counter-clockwise
            if self.idx > 0 {
                self.idx - 1
            } else {
                ROT-1
            }
        }
    }

    pub fn get_shape(&self) -> &Vec<(usize, usize)> {
        &self.rots[self.idx]
    }
}
