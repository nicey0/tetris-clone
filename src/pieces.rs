use std::f64::consts::PI;

pub const MAXX: i8 = 10;
pub const MAXY: i8 = 24;
pub const BOARDY: i8 = 22;

pub type Color = [f32; 4];
pub type Point = (i8, i8);
pub type FPoint = (f64, f64);

pub enum States {
    Stop,
    End,
    Nothing,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    color: Color,
    shape: [Point; 4],
    origin: FPoint,
}

impl Piece {
    pub fn new(color: Color, shape: [Point; 4], origin: FPoint) -> Self {
        Self {
            color,
            shape,
            origin,
        }
    }

    pub fn side(&mut self, c: i8, pieces: &Vec<ColPoint>) {
        // move in x axis
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after movement
        let mut origin: FPoint = self.origin; // origin after movement
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0 + c, self.shape[i].1);
            origin = (self.origin.0 + c as f64, self.origin.1);
            if shape[i].0 < 0
                || shape[i].0 >= MAXX
                || pieces.contains(&ColPoint {
                    point: shape[i],
                    color: [0.0; 4],
                })
            {
                return; // return if illegal position
            }
        }
        // set shape and origin to moved values
        self.shape = shape;
        self.origin = origin;
    }

    pub fn down(&mut self, c: i8, pieces: &Vec<ColPoint>) -> States {
        // move in y axis
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after movement
        let mut origin: FPoint = self.origin; // origin after movement
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0, self.shape[i].1 + c);
            origin = (self.origin.0, self.origin.1 + c as f64);
            if shape[i].1 >= MAXY
                || pieces.contains(&ColPoint {
                    point: shape[i],
                    color: [0.0; 4],
                })
            {
                return if shape[i].1 <= MAXY - BOARDY {
                    States::End // return End if illegal position & outside screen
                } else {
                    States::Stop // else return Stop
                };
            }
        }
        // set shape and origin to moved values & return Nothing
        self.shape = shape;
        self.origin = origin;
        States::Nothing
    }

    pub fn put_down(&mut self, pieces: &Vec<ColPoint>) {
        loop {
            match self.down(1, pieces) {
                States::Nothing => {}
                _ => break,
            }
        }
    }

    pub fn rotate(&mut self, pieces: &Vec<ColPoint>) {
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after rotation
        for i in 0..self.shape.len() {
            // math
            let coor = self.shape[i];
            let adj_x: f64 = (coor.0 as f64 - self.origin.0) * -1.0;
            let adj_y: f64 = (coor.1 as f64 - self.origin.1) * -1.0;
            let rot_x: f64 = adj_x * (PI / 2.0).cos() - adj_y * (PI / 2.0).sin();
            let rot_y: f64 = adj_x * (PI / 2.0).sin() + adj_y * (PI / 2.0).cos();
            shape[i].0 = (rot_x * -1.0 + self.origin.0).round() as i8;
            shape[i].1 = (rot_y * -1.0 + self.origin.1).round() as i8;
            // return if illegal position
            if shape[i].0 < 0
                || shape[i].0 >= MAXX
                || shape[i].1 < 0
                || shape[i].1 >= MAXY
                || pieces.contains(&ColPoint {
                    point: shape[i],
                    color: [0.0; 4],
                })
            {
                return;
            }
        }
        // set shape to moved value
        self.shape = shape;
    }

    pub fn get_shape(&self) -> &[Point; 4] {
        &self.shape
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}

/* piece constructors */
// [r, g, b, a]
// [shape]
// (origin x, origin y)

pub fn i() -> Piece {
    Piece::new(
        [0.0, 1.0, 1.0, 1.0],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        (1.5, 0.0),
    )
}

pub fn o() -> Piece {
    Piece::new(
        [1.0, 1.0, 0.0, 1.0],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        (0.5, 0.5),
    )
}

pub fn j() -> Piece {
    Piece::new(
        [0.0, 0.0, 1.0, 1.0],
        [(1, 0), (1, 1), (1, 2), (0, 2)],
        (1.0, 1.0),
    )
}

pub fn l() -> Piece {
    Piece::new(
        [1.0, 0.5, 0.0, 1.0],
        [(0, 0), (0, 1), (0, 2), (1, 2)],
        (0.0, 1.0),
    )
}

pub fn s() -> Piece {
    Piece::new(
        [0.0, 1.0, 0.0, 1.0],
        [(1, 0), (2, 0), (0, 1), (1, 1)],
        (1.0, 1.0),
    )
}

pub fn z() -> Piece {
    Piece::new(
        [1.0, 0.0, 0.0, 1.0],
        [(1, 1), (2, 1), (0, 0), (1, 0)],
        (1.0, 1.0),
    )
}

pub fn t() -> Piece {
    Piece::new(
        [0.5, 0.0, 0.8, 1.0],
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        (1.0, 1.0),
    )
}

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

pub struct ColPoint {
    pub point: Point,
    pub color: Color,
}

impl std::cmp::PartialEq<Point> for ColPoint {
    fn eq(&self, other: &Point) -> bool {
        self.point == *other
    }
}

impl std::cmp::PartialEq for ColPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
