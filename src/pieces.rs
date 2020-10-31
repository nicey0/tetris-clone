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
        Piece { color, rots, idx: 0 }
    }

    pub fn shift(&mut self, c: (i8, i8)) {
        for rot in self.rots.iter_mut() {
            for mut coor in rot {
                coor.0 += c.0;
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

    pub fn validate(&self, maxx: i8, maxy: i8) -> (i8, i8) {
        let (mut tx, mut ty);
        let (mut x, mut y) = (0, 0);
        for coor in self.get_shape() {
            // test x validity
            if coor.0 >= maxx {
                tx = maxx - coor.0 - 1;
                if tx < x { x = tx; }
            } else if coor.0 < 0 {
                tx = -coor.0;
                if tx > x { x = tx; }
            }
            // test y validity
            if coor.1 >= maxy {
                ty = maxy - coor.1 - 1;
                if ty < y { y = ty; }
            } else if coor.1 < 0 {
                ty = -coor.1;
                if ty > y { y = ty; }
            }
        }
        (x, y)
    }

    pub fn fix(&mut self, maxx: i8, maxy: i8) {
        self.shift(self.validate(maxx, maxy));
    }

    pub fn get_shape(&self) -> &Vec<(i8, i8)> {
        &self.rots[self.idx]
    }
}

pub fn o() -> Piece {
    Piece::new([1.0, 1.0, 0.0, 1.0], [
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    vec![(0, 0), (0, 1), (1, 0), (1, 1)],])
}

pub fn i() -> Piece {
    Piece::new([0.0, 1.0, 1.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (3, 1)],
    vec![(2, 0), (2, 1), (2, 2), (2, 3)],
    vec![(0, 2), (1, 2), (2, 2), (3, 2)],
    vec![(1, 0), (1, 1), (1, 2), (1, 3)],])
}

pub fn j() -> Piece {
    Piece::new([0.0, 0.0, 1.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (0, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 0)],
    vec![(0, 1), (1, 1), (2, 1), (2, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 2)],])
}

pub fn l() -> Piece {
    Piece::new([1.0, 0.5, 0.0, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (2, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 2)],
    vec![(0, 1), (1, 1), (2, 1), (0, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 0)],])
}

pub fn s() -> Piece {
    Piece::new([0.0, 1.0, 0.0, 1.0], [
    vec![(0, 1), (1, 1), (1, 0), (2, 0)],
    vec![(1, 0), (1, 1), (2, 1), (2, 2)],
    vec![(0, 2), (1, 2), (1, 1), (2, 1)],
    vec![(0, 0), (0, 1), (1, 1), (1, 2)],])
}

pub fn z() -> Piece {
    Piece::new([1.0, 0.0, 0.0, 1.0], [
    vec![(0, 0), (1, 0), (1, 1), (2, 1)],
    vec![(1, 1), (1, 2), (2, 1), (2, 0)],
    vec![(0, 1), (1, 1), (1, 2), (2, 2)],
    vec![(1, 0), (1, 1), (0, 1), (0, 2)],])
}

pub fn t() -> Piece {
    Piece::new([0.5, 0.0, 0.8, 1.0], [
    vec![(0, 1), (1, 1), (2, 1), (1, 0)],
    vec![(1, 0), (1, 1), (1, 2), (2, 1)],
    vec![(0, 1), (1, 1), (2, 1), (1, 2)],
    vec![(1, 0), (1, 1), (1, 2), (0, 1)],])
}
