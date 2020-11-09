mod colpoint;
mod conf;
mod game;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod update;
mod util;
use game::Game;
use piston_window::*;
use util::*;

fn main() {
    let mut game = Game::new();
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
