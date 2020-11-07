extern crate rand;
use rand::seq::SliceRandom;

mod pieces;
use pieces::*;

use std::{io, thread, time};
use std::collections::HashMap;

const MAXX: i8 = 10;
const MAXY: i8 = 26;
const BOARDY: i8 = 22;

const PIECES: [fn(i8, i8, i8) -> Piece; 7] = [i, o, j, l, s, z, t];

const RATE: i8 = 1; // lower rate = faster drop

fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()(MAXX, MAXY, BOARDY)
}

fn check_clear(pieces: &mut Vec<Point>) {
    let mut cleared: Vec<i8> = Vec::with_capacity(BOARDY as usize);
    let mut length: HashMap<i8, i8> = HashMap::new();
    for i in 0..pieces.len() {
        *length.entry(pieces[i].1).or_insert(0) += 1;
        if length[&pieces[i].1] >= MAXX {
            cleared.push(pieces[i].1);
        }
    }
    let mut higher: i8 = 0;
    *pieces = pieces.iter_mut().filter_map(|coor| {
        higher = 0;
        for &y in cleared.iter() {
            if coor.1 == y {
                return None
            } else if coor.1 < y {
                higher += 1;
            }
        }
        Some((coor.0, coor.1 + higher))
    }).collect();
}

fn main() {
    // input
    let stdin = io::stdin();
    let mut input = String::new();

    // grav rate
    let mut rate = 0;
    // Vector holding still pieces
    let mut pieces: Vec<Point> = Vec::new();
    // Vector holding still pieces' colors (I know it's weird, deal with it)
    let mut colors: Vec<Color> = Vec::new();
    let mut p = random_piece();
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear scr
        println!("{}", rate);
        print_board(&p, &pieces);
        match stdin.read_line(&mut input) {
            Ok(_) => {
                if      &input.trim()[..] == "h" { p.side(-1, &pieces); }
                else if &input.trim()[..] == "l" { p.side(1, &pieces); }
                else if &input.trim()[..] == "r" { p.rotate(&pieces); }
                input = String::new();
            },
            _ => {},
        }

        //p.side(1, &pieces);
        //p.rotate(&pieces);
        rate += 1;
        if rate == RATE {
            rate = 0;
            match p.down(1, &pieces) { // gravity
                States::Stop => { // if illegal position, stop moving and instantiate new piece
                    for &s in p.get_shape().iter() {
                        pieces.push(s);
                        colors.push(*p.get_color());
                    }
                    check_clear(&mut pieces);
                    p = random_piece();
                }, // if outside screen & illegal position, end game
                States::End => break,
                _ => {},
            };
        }
        thread::sleep(time::Duration::from_millis(20));
    }
}

fn print_board(p: &Piece, pieces: &Vec<Point>) {
    let s =  p.get_shape();
    println!("{:?}", s);
    for y in MAXY-BOARDY..MAXY {
        for x in 0..MAXX {
            if s.contains(&(x, y)) {
                print!("██");
            } else if pieces.contains(&(x, y)) {
                print!("▒▒");
            } else {
                print!("..");
            }
        }
        println!("");
    }
}
