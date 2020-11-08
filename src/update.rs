use rand::seq::SliceRandom;
use std::collections::HashMap;

use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;

pub fn update(
    p: &mut Piece,
    shadow: &mut Shadow,
    next: &mut Piece,
    score: &mut u32,
    pieces: &mut Vec<ColPoint>,
    rate: &mut i8,
) -> bool {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", score);
    *rate += 1;
    if *rate == RATE {
        *rate = 0;
        if !move_down(p, next, pieces, score) {
            return false;
        }
    }
    *shadow = Shadow::new(p);
    shadow.put_down(pieces);
    true
}

fn move_down(p: &mut Piece, next: &mut Piece, pieces: &mut Vec<ColPoint>, score: &mut u32) -> bool {
    match p.down(1, pieces) {
        // gravity
        States::Stop => {
            // if illegal position, stop moving and instantiate new piece
            for &s in p.get_shape().iter() {
                pieces.push(ColPoint {
                    point: s,
                    color: p.get_color(),
                });
            }
            *score += match check_clear(pieces) {
                0 => 0,
                1 => 120,
                2 => 200,
                3 => 600,
                _ => 2400,
            };
            *p = Piece::new_from_next(next);
            *next = random_piece();
        } // if outside screen & illegal position, end game
        States::End => return false,
        _ => {}
    };
    true
}

pub fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()()
}

fn check_clear(pieces: &mut Vec<ColPoint>) -> usize {
    // Vector of cleared y's
    let mut cleared: Vec<i8> = Vec::with_capacity(BOARDY as usize);
    // HashMap holding {y: width}
    let mut widths: HashMap<i8, i8> = HashMap::new();
    for i in 0..pieces.len() {
        *widths.entry(pieces[i].point.1).or_insert(0) += 1;
        if widths[&pieces[i].point.1] >= MAXX {
            // if line is cleared, add y to cleared
            cleared.push(pieces[i].point.1);
        }
    }
    let mut higher: i8 = 0;
    *pieces = pieces
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
