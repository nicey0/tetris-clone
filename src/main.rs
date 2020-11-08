use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent};

mod colpoint;
mod conf;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod update;
mod util;
use colpoint::ColPoint;
use conf::*;
use key::*;
use pieces::Piece;
use render::*;
use shadow::Shadow;
use update::*;
use util::*;

fn main() {
    // grav rate
    let mut rate = 0;
    // Vector holding still pieces
    let mut pieces: Vec<ColPoint> = Vec::new();
    let mut p = random_piece();
    let mut shadow = Shadow::new(&p);
    let mut next = random_piece();
    let mut score: u32 = 0;

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
            if !update(
                &mut p,
                &mut shadow,
                &mut next,
                &mut score,
                &mut pieces,
                &mut rate,
            ) {
                break;
            }
        } else if let Some(args) = e.render_args() {
            // RENDER
            gl.draw(args.viewport(), |c, g| {
                clear([0.1, 0.1, 0.1, 1.0], g);
                draw_well(&c, g);
                draw_next(&c, g, &next);
                draw_pieces(&c, g, &p, &shadow, &pieces);
            });
        } else if let Some(button) = e.press_args() {
            handle_button(button, &mut p, &pieces);
        }
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
