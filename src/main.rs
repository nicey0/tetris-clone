extern crate rand;
use rand::seq::SliceRandom;

extern crate glutin_window;
use glutin_window::GlutinWindow as Window;
extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};
extern crate piston;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Key, PressEvent};
extern crate graphics;
use graphics::*;

mod colpoint;
mod conf;
mod piece_defs;
mod pieces;
mod shadow;
mod util;
use colpoint::ColPoint;
use conf::*;
use pieces::Piece;
use shadow::Shadow;
use util::*;

use std::collections::HashMap;

fn main() {
    // grav rate
    let mut rate = 0;
    // Vector holding still pieces
    let mut pieces: Vec<ColPoint> = Vec::new();
    let mut p = random_piece();
    let mut shadow = Shadow::new(&p);
    let mut next = random_piece();

    // GUI setup
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Tetris Clone", WINSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    // Main loop
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            // UPDATE
            if !update(&mut p, &mut next, &mut pieces, &mut rate) {
                break;
            }
        } else if let Some(args) = e.render_args() {
            // RENDER
            render(&mut gl, &args, &mut p, &next, &mut shadow, &mut pieces);
        } else if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    handle_key(key, &mut p, &pieces);
                }
                _ => {}
            }
        }
    }
}

fn random_piece() -> Piece {
    PIECES.choose(&mut rand::thread_rng()).unwrap()()
}

fn check_clear(pieces: &mut Vec<ColPoint>) {
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
}

fn update(p: &mut Piece, next: &mut Piece, pieces: &mut Vec<ColPoint>, rate: &mut i8) -> bool {
    print!("\x1B[2J\x1B[1;1H");
    *rate += 1;
    if *rate == RATE {
        *rate = 0;
        match p.down(1, &pieces) {
            // gravity
            States::Stop => {
                // if illegal position, stop moving and instantiate new piece
                for &s in p.get_shape().iter() {
                    pieces.push(ColPoint {
                        point: s,
                        color: p.get_color(),
                    });
                }
                check_clear(pieces);
                *p = Piece::new_from_next(next);
                *next = random_piece();
            } // if outside screen & illegal position, end game
            States::End => return false,
            _ => {}
        };
    }
    true
}

fn render(
    gl: &mut GlGraphics,
    args: &RenderArgs,
    p: &mut Piece,
    next: &Piece,
    shadow: &mut Shadow,
    pieces: &mut Vec<ColPoint>,
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
                                - ((5 - next.get_width()) as f64 / 2.0 * CELLSIZE),
                            CELLSIZE,
                            CELLSIZE,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
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

fn handle_key(key: Key, p: &mut Piece, pieces: &Vec<ColPoint>) {
    match key {
        Key::Right => p.side(1, &pieces),
        Key::Left => p.side(-1, &pieces),
        Key::Up => p.rotate(&pieces),
        Key::Down => p.put_down(&pieces),
        _ => {}
    }
}

fn print_board(p: &Piece, pieces: &Vec<ColPoint>) {
    let s = p.get_shape();
    println!("{:?}", s);
    for y in MAXY - BOARDY..MAXY {
        for x in 0..MAXX {
            if s.contains(&(x, y)) {
                print!("██");
            } else if pieces.contains(&ColPoint {
                point: (x, y),
                color: [0.0; 4],
            }) {
                print!("▒▒");
            } else {
                print!("..");
            }
        }
        println!("");
    }
}
