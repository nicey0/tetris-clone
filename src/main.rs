use piston_window::*;

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
    let mut mrate = INIRATE;
    let mut cl = 0;
    let mut rate = 0;
    // Vector holding still pieces
    let mut pieces: Vec<ColPoint> = Vec::new();
    let mut p = random_piece();
    let mut shadow = Shadow::new(&p);
    let mut next = random_piece();
    let mut score: u32 = 0;

    // GUI setup
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Tetris Clone", WINSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    let font_data: &[u8] = include_bytes!("../fonts/FiraSans-Regular.ttf");
    let mut cache = Glyphs::from_bytes(
        font_data,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();

    // Main loop
    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            // UPDATE
            if !update(
                &mut p,
                &mut shadow,
                &mut next,
                &mut score,
                &mut pieces,
                &mut rate,
                &mut mrate,
                &mut cl,
            ) {
                break;
            }
        } else if let Some(_) = e.render_args() {
            // RENDER
            window.draw_2d(&e, |c, g, _d| {
                clear([0.1, 0.1, 0.1, 1.0], g);
                draw_well(&c, g);
                draw_next(&c, g, &next);
                draw_score(&c, g, &score, &mut cache);
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
