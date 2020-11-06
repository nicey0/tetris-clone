extern crate rand;

mod pieces;

use pieces::*;
use std::{thread, time};
use rand::seq::SliceRandom;

const MAXX: i8 = 10;
const MAXY: i8 = 26;
const BOARDY: i8 = 22;
const PIECES: [fn(i8, i8, i8) -> Piece; 7] = [i, o, j, l, s, z, t];

fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()(MAXX, MAXY, BOARDY)
}

fn main() {
    // Vector holding still pieces
    let mut pieces: Vec<Point> = Vec::new();
    // Vector holding still pieces' colors (I know it's weird, deal with it)
    let mut colors: Vec<Color> = Vec::new();
    let mut p = random_piece();
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear scr
        print_board(&p, &pieces);
        match p.down(1, &pieces) { // gravity
            States::Stop => { // if illegal position, stop moving and instantiate new piece
                for &s in p.get_shape().iter() {
                    pieces.push(s); colors.push(*p.get_color())
                }
                p = random_piece();
            }, // if outside screen & illegal position, end game
            States::End => break,
            _ => {},
        };
        p.side(-1, &pieces);
        p.rotate(&pieces);
        thread::sleep(time::Duration::from_millis(120));
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
