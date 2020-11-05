mod pieces;

use pieces::{Piece, o, i, j, l, s, z, t};
use std::{thread, time};

const MAXX: i8 = 10;
const MAXY: i8 = 22;

fn main() {
    let mut pieces: Vec<Piece> = Vec::new();
    let mut piece = o(MAXX, MAXY);
    loop {
        print!("\x1B[2J\x1B[1;1H");
        if !piece.step(1) {
            // TODO: switch to new piece
            pieces.push(piece.clone());
            piece = i(MAXX, MAXY);
        }
        for y in 0..MAXY {
            for x in 0..MAXX {
                if piece.get_shape().contains(&(x, y)) {
                    print!("X ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}
