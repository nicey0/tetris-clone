extern crate rand;
use rand::seq::SliceRandom;

extern crate glutin_window;
use glutin_window::GlutinWindow as Window;
extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};
extern crate piston;
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::{Button, PressEvent, Key};
use piston::window::WindowSettings;
extern crate graphics;
use graphics::*;

mod pieces;
use pieces::*;

use std::{io, thread, time};
use std::collections::HashMap;

const CELLSIZE: f64 = 20.0;
const WINSIZE: [f64; 2] = [MAXX as f64 * CELLSIZE, BOARDY as f64 * CELLSIZE];
const TOP_PAD: f64 = (MAXY - BOARDY) as f64 * CELLSIZE;

const PIECES: [fn() -> Piece; 7] = [i, o, j, l, s, z, t];

const RATE: i8 = 25; // lower rate = faster drop

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
        if widths[&pieces[i].point.1] >= MAXX { // if line is cleared, add y to cleared
            cleared.push(pieces[i].point.1);
        }
    }
    let mut higher: i8 = 0;
    *pieces = pieces.iter().filter_map(|coor| {
        higher = 0;
        for &y in cleared.iter() {
            if coor.point.1 == y {
                return None
            } else if coor.point.1 < y {
                higher += 1;
            }
        }
        Some(ColPoint{ point: (coor.point.0, coor.point.1 + higher), color: coor.color })
    }).collect();
}

fn main() {
    // grav rate
    let mut rate = 0;
    // Vector holding still pieces
    let mut pieces: Vec<ColPoint> = Vec::new();
    let mut p = random_piece();

    // GUI setup
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Tetris Clone", WINSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build().unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    // Main loop
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            print!("\x1B[2J\x1B[1;1H");
            // UPDATE
            rate += 1;
            if rate == RATE {
                rate = 0;
                match p.down(1, &pieces) { // gravity
                    States::Stop => { // if illegal position, stop moving and instantiate new piece
                        for &s in p.get_shape().iter() {
                            pieces.push(ColPoint { point: s, color: p.get_color() });
                        }
                        check_clear(&mut pieces);
                        p = random_piece();
                    }, // if outside screen & illegal position, end game
                    States::End => break,
                    _ => {},
                };
            }
        } else if let Some(args) = e.render_args() {
            // RENDER
            gl.draw(args.viewport(), |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let s = p.get_shape();
                for y in MAXY-BOARDY..MAXY {
                    for x in 0..MAXX {
                        if s.contains(&(x, y)) {
                            // draw block
                            rectangle(
                                p.get_color(),
                                [
                                    x as f64 * CELLSIZE,
                                    y as f64 * CELLSIZE - TOP_PAD,
                                    CELLSIZE,
                                    CELLSIZE
                                ], c.transform, g
                            );
                        } else if pieces.contains(&ColPoint{ point: (x, y), color: [0.0; 4] }) {
                            // draw block
                            rectangle(
                                pieces[pieces.iter().position(|e| e == &(x, y)).unwrap()].color,
                                [
                                    x as f64 * CELLSIZE,
                                    y as f64 * CELLSIZE - TOP_PAD,
                                    CELLSIZE,
                                    CELLSIZE
                                ], c.transform, g
                            );
                        }
                    }
                }
            });
        } else if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    match key {
                    Key::Right => { p.side(1, &pieces) },
                    Key::Left => { p.side(-1, &pieces) },
                    Key::Up => { p.rotate(&pieces) },
                    Key::Down => { p.put_down(&pieces) },
                    _ => {},
                    }
                },
                _ => {},
            }
        }
    }
}

//fn print_board(p: &Piece, pieces: &Vec<ColPoint>) {
    //let s =  p.get_shape();
    //println!("{:?}", s);
    //for y in MAXY-BOARDY..MAXY {
        //for x in 0..MAXX {
            //if s.contains(&(x, y)) {
                //print!("██");
            //} else if pieces.contains(&(x, y)) {
                //print!("▒▒");
            //} else {
                //print!("..");
            //}
        //}
        //println!("");
    //}
//}
