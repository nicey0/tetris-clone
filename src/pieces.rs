use std::f64::consts::PI;

pub type Color = [f32; 4];
pub type Point = (i8, i8);
pub type FPoint = (f64, f64);

pub enum States {
    Stop,
    End,
    Nothing
}

#[derive(Clone, Debug)]
pub struct Piece {
    color: Color,
    shape: [Point; 4],
    origin: FPoint,
    maxx: i8,
    maxy: i8,
    boardy: i8,
}

impl Piece {
    pub fn new(color: Color, shape: [Point; 4], origin: FPoint, maxx: i8, maxy: i8, boardy: i8) -> Self {
        Self {
            color,
            shape,
            origin,
            maxx,
            maxy,
            boardy,
        }
    }

    pub fn side(&mut self, c: i8, pieces: &Vec<Point>) {
        // move in x axis
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after movement
        let mut origin: FPoint = self.origin; // origin after movement
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0 + c, self.shape[i].1);
            origin = (self.origin.0 + c as f64, self.origin.1);
            if shape[i].0 < 0 || shape[i].0 >= self.maxx 
                    || pieces.contains(&shape[i]) {
                return // return if illegal position
            }
        }
        // set shape and origin to moved values
        self.shape = shape;
        self.origin = origin;
    }

    pub fn down(&mut self, c: i8, pieces: &Vec<Point>) -> States {
        // move in y axis
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after movement
        let mut origin: FPoint = self.origin; // origin after movement
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0, self.shape[i].1 + c);
            origin = (self.origin.0, self.origin.1 + c as f64);
            if shape[i].1 >= self.maxy  || pieces.contains(&shape[i]) {
                return if shape[i].1 <= self.maxy - self.boardy - 2 {
                    States::End // return End if illegal position & outside screen
                } else {
                    States::Stop // else return Stop
                }
            }
        }
        // set shape and origin to moved values & return Nothing
        self.shape = shape;
        self.origin = origin;
        States::Nothing
    }

    pub fn rotate(&mut self, pieces: &Vec<Point>) {
        let mut shape: [Point; 4] = [(0, 0); 4]; // shape after rotation
        for i in 0..self.shape.len() {
            // math
            let coor = self.shape[i];
            let adj_x: f64 = (coor.0 as f64 - self.origin.0) * -1.0;
            let adj_y: f64 = (coor.1 as f64 - self.origin.1) * -1.0;
            let rot_x: f64 = adj_x * (PI/2.0).cos() - adj_y * (PI/2.0).sin();
            let rot_y: f64 = adj_x * (PI/2.0).sin() + adj_y * (PI/2.0).cos();
            shape[i].0 = (rot_x * -1.0 + self.origin.0).round() as i8;
            shape[i].1 = (rot_y * -1.0 + self.origin.1).round() as i8;
            // return if illegal position
            if shape[i].0 < 0 || shape[i].0 >= self.maxx || shape[i].1 < 0 || shape[i].1 >= self.maxy ||
                    pieces.contains(&shape[i]) {
                return
            }
        }
        // set shape to moved value
        self.shape = shape;
    }

    pub fn get_shape(&self) -> &[Point; 4] {
        &self.shape
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }
}


pub fn i(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([1.0, 1.0, 0.0, 1.0],
               [(0, 0), (1, 0), (2, 0), (3, 0)],
               (1.5, 0.0),
               maxx, maxy, boardy)
}

pub fn o(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([0.0, 1.0, 1.0, 1.0],
               [(0, 0), (1, 0), (0, 1), (1, 1)],
               (0.5, 0.5),
               maxx, maxy, boardy)
}

pub fn j(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([0.0, 0.0, 1.0, 1.0],
               [(1, 0), (1, 1), (1, 2), (0, 2)],
               (1.0, 1.0),
               maxx, maxy, boardy)
}

pub fn l(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([1.0, 0.5, 0.0, 1.0],
               [(0, 0), (0, 1), (0, 2), (1, 2)],
               (0.0, 1.0),
               maxx, maxy, boardy)
}

pub fn s(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([0.0, 1.0, 0.0, 1.0],
               [(1, 0), (2, 0), (0, 1), (1, 1)],
               (1.0, 1.0),
               maxx, maxy, boardy)
}

pub fn z(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([1.0, 0.0, 0.0, 1.0],
               [(1, 1), (2, 1), (0, 0), (1, 0)],
               (1.0, 1.0),
               maxx, maxy, boardy)
}

pub fn t(maxx: i8, maxy: i8, boardy: i8) -> Piece {
    Piece::new([0.5, 0.0, 0.8, 1.0],
               [(1, 0), (0, 1), (1, 1), (2, 1)],
               (1.0, 1.0),
               maxx, maxy, boardy)
}

