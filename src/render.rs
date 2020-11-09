use piston_window::glyph_cache::rusttype::GlyphCache;
use piston_window::*;

use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;

pub fn draw_well(c: &Context, g: &mut G2d) {
    rectangle(
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, WELLWIDTH, WINSIZE.1],
        c.transform,
        g,
    );
}

pub fn draw_next(c: &Context, g: &mut G2d, next: &Piece) {
    // draw next piece background bit
    rectangle(
        [0.3, 0.3, 0.3, 1.0],
        [WELLWIDTH + CELLSIZE, CELLSIZE, NWIDTH, NHEIGHT],
        c.transform,
        g,
    );
    // draw next piece
    for y in 0..4 {
        for x in 0..4 {
            if next.get_shape().contains(&(x, y)) {
                rectangle(
                    next.get_color(),
                    [
                        WELLWIDTH + CELLSIZE + x as f64 * CELLSIZE + NWIDTH / 2.0
                            - (next.get_width() as f64 / 2.0 * CELLSIZE),
                        CELLSIZE + y as f64 * CELLSIZE + NHEIGHT / 2.0
                            - ((next.get_height()) as f64 / 2.0 * CELLSIZE),
                        CELLSIZE,
                        CELLSIZE,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }
}

pub fn draw_letter(c: &Context, g: &mut G2d, i: usize, letter: char, font: &mut Glyphs) {
    //println!("{:?}", score);
    Text::new_color([1.0; 4], CELLSIZE as u32)
        .draw(
            &letter.to_string(),
            font,
            &DrawState::default(),
            c.transform.trans(
                WELLWIDTH + CELLSIZE + i as f64 * (CELLSIZE / 1.6),
                CELLSIZE * 2.0 + NHEIGHT + CELLSIZE / 2.0,
            ),
            g,
        )
        .expect("error drawing text!");
}

pub fn draw_pieces(c: &Context, g: &mut G2d, p: &Piece, shadow: &Shadow, pieces: &Vec<ColPoint>) {
    let s = p.get_shape();
    for y in MAXY - BOARDY..MAXY {
        for x in 0..MAXX {
            if s.contains(&(x, y)) {
                // draw block
                rectangle(
                    p.get_color(),
                    [
                        x as f64 * CELLSIZE,
                        y as f64 * CELLSIZE - TOP_PAD,
                        CELLSIZE,
                        CELLSIZE,
                    ],
                    c.transform,
                    g,
                );
            } else if pieces.contains(&ColPoint {
                point: (x, y),
                color: [0.0; 4],
            }) {
                // draw block
                rectangle(
                    pieces[pieces.iter().position(|e| e == &(x, y)).unwrap()].color,
                    [
                        x as f64 * CELLSIZE,
                        y as f64 * CELLSIZE - TOP_PAD,
                        CELLSIZE,
                        CELLSIZE,
                    ],
                    c.transform,
                    g,
                );
            } else if shadow.shape.contains(&(x, y)) {
                rectangle(
                    SHDOW,
                    [
                        x as f64 * CELLSIZE,
                        y as f64 * CELLSIZE - TOP_PAD,
                        CELLSIZE,
                        CELLSIZE,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }
}
