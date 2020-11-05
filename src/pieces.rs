const ROT: usize = 4;

pub type Rotations = [Vec<(i8, i8)>; ROT];

#[derive(Clone, Debug)]
pub struct Piece {
    color: [f32; 4],
    rots: Rotations,
    idx: usize,
    maxx: i8,
    maxy: i8,
}

impl Piece {
    pub fn new(color: [f32; 4], rots: Rotations, maxx: i8, maxy: i8) -> Self {
        Piece { color, rots, idx: 0, maxx, maxy }
    }

    pub fn shift(&mut self, c: (i8, i8)) {
        for rot in self.rots.iter_mut() {
            for mut coor in rot {
                coor.0 += c.0;
                if coor.0 >= self.maxx {
                    coor.0 += self.maxx - coor.0 - 2;
                }
                coor.1 += c.1;
            }
        }
    }

    pub fn rotate(&mut self, dir: bool) {
        self.idx = if dir { // clockwise
            if self.idx < ROT-1 { self.idx + 1 }
            else { 0 }
        } else { // counter-clockwise
            if self.idx > 0 { self.idx - 1 }
            else { ROT-1 }
        }
    }

    pub fn get_shape(&self) -> &Vec<(i8, i8)> {
        &self.rots[self.idx]
    }

    pub fn step(&mut self, x: i8) -> bool {
        for (_, y) in self.get_shape().iter() {
            if y == &(self.maxy-1) {
                return false
            }
        }
        self.shift((x, 1));
        true
    }
}

pub fn o(maxx: i8, maxy: i8) -> Piece {
    Piece::new([1.0, 1.0, 0.0, 1.0], [
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],],
    maxx, maxy)
}

pub fn i(maxx: i8, maxy: i8) -> Piece {
    Piece::new([0.0, 1.0, 1.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (3, 1)],
    vec![(2, 0), (2, 1), (2, 2), (2, 3)],
    vec![(0, 2), (1, 2), (2, 2), (3, 2)],
    vec![(1, 0), (1, 1), (1, 2), (1, 3)],],
    maxx, maxy)
}

pub fn j(maxx: i8, maxy: i8) -> Piece {
    Piece::new([0.0, 0.0, 1.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (0, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 0)],
    vec![(0, 1), (1, 1), (2, 1), (2, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 2)],],
    maxx, maxy)
}

pub fn l(maxx: i8, maxy: i8) -> Piece {
    Piece::new([1.0, 0.5, 0.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (2, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 2)],
    vec![(0, 1), (1, 1), (2, 1), (0, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 0)],],
    maxx, maxy)
}

pub fn s(maxx: i8, maxy: i8) -> Piece {
    Piece::new([0.0, 1.0, 0.0, 1.0], [
    vec![(0, 1), (1, 1), (1, 0), (2, 0)],
    vec![(1, 0), (1, 1), (2, 1), (2, 2)],
    vec![(0, 2), (1, 2), (1, 1), (2, 1)],
    vec![(0, 0), (0, 1), (1, 1), (1, 2)],],
    maxx, maxy)
}

pub fn z(maxx: i8, maxy: i8) -> Piece {
    Piece::new([1.0, 0.0, 0.0, 1.0], [
    vec![(0, 0), (1, 0), (1, 1), (2, 1)],
    vec![(1, 1), (1, 2), (2, 1), (2, 0)],
    vec![(0, 1), (1, 1), (1, 2), (2, 2)],
    vec![(1, 0), (1, 1), (0, 1), (0, 2)],],
    maxx, maxy)
}

pub fn t(maxx: i8, maxy: i8) -> Piece {
    Piece::new([0.5, 0.0, 0.8, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (1, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 1)],
    vec![(0, 1), (1, 1), (2, 1), (1, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 1)],],
    maxx, maxy)
}
