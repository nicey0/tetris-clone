extern crate rand;

mod pieces;

use pieces::*;
use std::{thread, time};
use rand::seq::SliceRandom;
use std::io;

const MAXX: i8 = 10;
const MAXY: i8 = 26;
const BOARDY: i8 = 22;
const PIECES: [fn(i8, i8, i8) -> Piece; 7] = [i, o, j, l, s, z, t];
const RATE: i16 = 4;

fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()(MAXX, MAXY, BOARDY)
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
                if &input.trim()[..] == "h" { p.side(-1, &pieces); }
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
