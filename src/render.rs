extern crate glutin_window;
extern crate opengl_graphics;
use opengl_graphics::GlGraphics;
extern crate piston;
use piston::input::RenderArgs;
extern crate graphics;
use graphics::*;

use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;

fn draw_next(c: Context, g: &mut GlGraphics, next: &Piece) {
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

pub fn render(
    gl: &mut GlGraphics,
    args: &RenderArgs,
    p: &Piece,
    next: &Piece,
    shadow: &mut Shadow,
    pieces: &Vec<ColPoint>,
) {
    *shadow = Shadow::new(p);
    shadow.put_down(pieces);
    gl.draw(args.viewport(), |c, g| {
        clear([0.1, 0.1, 0.1, 1.0], g);
        // draw well bit
        rectangle(
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, WELLWIDTH, WINSIZE.1],
            c.transform,
            g,
        );
        draw_next(c, g, next);
        // draw board
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
                        SHADOWCOLOR,
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
    });
}
