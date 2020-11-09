mod colpoint;
mod conf;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod state;
mod update;
mod util;
use state::State;

fn main() {
    let game = State::new();

    // GUI setup
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Tetris Clone", WINSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    window.set_ups(FPS);
    window.set_max_fps(FPS);
    let mut font;

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
                draw_pieces(&c, g, &p, &shadow, &pieces);
            });
            for (i, ch) in score.to_string().chars().enumerate() {
                font = match window.load_font("fonts/FiraSans-Regular.ttf") {
                    Ok(f) => f,
                    Err(e) => panic!("{}", e),
                };
                window.draw_2d(&e, |c, g, _d| draw_letter(&c, g, i, ch, &mut font));
            }
        } else if let Some(button) = e.press_args() {
            handle_button(button, &mut p, &pieces, &mut rate, &mrate);
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
