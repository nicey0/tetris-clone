mod colpoint;
mod conf;
mod driver;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod state;
mod update;
mod util;
use conf::*;
use piston_window::*;
use state::State;
use util::*;

fn main() {
    let mut game = State::new();
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Tetris Clone", WINSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    //window.set_ups(FPS);
    //window.set_max_fps(FPS);
    game.mainloop(&mut window);
}
