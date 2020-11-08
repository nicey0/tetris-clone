use super::colpoint::ColPoint;
use super::conf::*;
use super::util::*;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    color: Color,
    shape: [Point; 4],
    origin: FPoint,
    rotates: bool,
    width: i8,
    height: i8,
}

impl Piece {
    fn adjust(x: f64, o: f64) -> i8 {
        (x + ((MAXX as f64 / 2.0) - o).round()) as i8 - 1
    }

    pub fn new(
        color: Color,
        shape: [Point; 4],
        origin: FPoint,
        rotates: bool,
        width: i8,
        height: i8,
    ) -> Self {
        Self {
            color,
            shape,
            origin,
            rotates,
            width,
            height,
        }
    }

    pub fn new_from_next(next: &Self) -> Self {
        let mut adj: [Point; 4] = [(0, 0); 4];
        next.shape
            .iter()
            .enumerate()
            .for_each(|(i, &p)| adj[i] = (Piece::adjust(p.0 as f64, next.origin.0), p.1));
        Self {
            color: next.color,
            shape: adj,
            origin: (
                Piece::adjust(next.origin.0, next.origin.0) as f64,
                next.origin.1,
            ),
            width: next.width,
            height: next.height,
            rotates: next.rotates,
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
        let mut col: bool = false;
        let mut outside: bool = false;
        for i in 0..self.shape.len() {
            shape[i] = (self.shape[i].0, self.shape[i].1 + c);
            origin = (self.origin.0, self.origin.1 + c as f64);
            if shape[i].1 >= MAXY
                || pieces.contains(&ColPoint {
                    point: shape[i],
                    color: [0.0; 4],
                })
            {
                col = true;
            }
            if shape[i].1 <= MAXY - BOARDY {
                outside = true;
            }
        }
        return if outside && col {
            States::End // return End if illegal position & outside screen
        } else if col {
            States::Stop // else if only illegal position return Stop
        } else {
            // else set shape and origin to moved values & return Nothing
            self.shape = shape;
            self.origin = origin;
            States::Nothing
        };
    }

    pub fn put_down(&mut self, pieces: &Vec<ColPoint>) {
        // keep moving down until collision
        loop {
            match self.down(1, pieces) {
                States::Nothing => {}
                _ => break,
            }
        }
    }

    pub fn rotate(&mut self, pieces: &Vec<ColPoint>) {
        if !self.rotates {
            return;
        }
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

    pub fn get_origin(&self) -> FPoint {
        self.origin
    }

    pub fn get_width(&self) -> i8 {
        self.width
    }

    pub fn get_height(&self) -> i8 {
        self.height
    }
}
