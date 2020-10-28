const ROT: usize = 4;

pub type Rotations = [Vec<(i8, i8)>; ROT];

#[derive(Clone, Debug)]
pub struct Piece {
    color: [f32; 4],
    rots: Rotations,
    idx: usize,
}

impl Piece {
    pub fn new(color: [f32; 4], rots: Rotations) -> Self {
        Piece {color, rots, idx: 0}
    }

    pub fn shift(&mut self, x: i8, y: i8) {
        for rot in self.rots.iter_mut() {
            for mut coor in rot {
                coor.0 += x;
                coor.1 += y;
            }
        }
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

    pub fn get_shape(&self) -> &Vec<(i8, i8)> {
        &self.rots[self.idx]
    }
}
