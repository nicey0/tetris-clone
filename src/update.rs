use std::collections::HashMap;

use super::colpoint::ColPoint;
use super::conf::*;
use super::game::Game;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;

impl Game {
    pub fn update(&mut self) -> States {
        // create shadow and put it down
        self.update_shadow();
        self.rate += 1;
        if self.rate >= self.mrate {
            self.rate = 0;
            if !self.move_down() {
                return States::End;
            };
        }
        States::Nothing
    }

    fn update_shadow(&mut self) {
        self.shadow = Shadow::new(&self.p);
        self.shadow.put_down(&self.pieces);
    }

    fn move_down(&mut self) -> bool {
        match self.p.down(1, &self.pieces) {
            // gravity
            States::Stop => {
                // if illegal position, stop moving and instantiate new piece
                for &s in self.p.get_shape().iter() {
                    self.pieces.push(ColPoint {
                        point: s,
                        color: self.p.get_color(),
                    });
                }
                self.score += match self.check_clear() {
                    1 => {
                        self.cl += 1;
                        40 * (self.level + 1) as u32
                    }
                    2 => {
                        self.cl += 2;
                        100 * (self.level + 1) as u32
                    }
                    3 => {
                        self.cl += 3;
                        300 * (self.level + 1) as u32
                    }
                    4 => {
                        self.cl += 4;
                        1200 * (self.level + 1) as u32
                    }
                    _ => 0,
                };
                if self.cl >= CLEARS && self.mrate >= RL {
                    self.cl = 0;
                    self.mrate -= RL;
                    self.level += 1;
                }
                self.p = Piece::new_from_next(&self.next);
                self.next = random_piece();
            } // if outside screen & illegal position, end game
            States::End => return false,
            _ => {}
        };
        true
    }

    fn check_clear(&mut self) -> usize {
        // Vector of cleared y's
        let mut cleared: Vec<i8> = Vec::with_capacity(BOARDY as usize);
        // HashMap holding {y: width}
        let mut widths: HashMap<i8, i8> = HashMap::new();
        for i in 0..self.pieces.len() {
            *widths.entry(self.pieces[i].point.1).or_insert(0) += 1;
            if widths[&self.pieces[i].point.1] >= MAXX {
                // if line is cleared, add y to cleared
                cleared.push(self.pieces[i].point.1);
            }
        }
        let mut higher: i8 = 0;
        self.pieces = self
            .pieces
            .iter()
            .filter_map(|coor| {
                higher = 0;
                for &y in cleared.iter() {
                    if coor.point.1 == y {
                        return None;
                    } else if coor.point.1 < y {
                        higher += 1;
                    }
                }
                Some(ColPoint {
                    point: (coor.point.0, coor.point.1 + higher),
                    color: coor.color,
                })
            })
            .collect();
        cleared.len()
    }
}
