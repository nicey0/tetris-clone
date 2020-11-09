use super::colpoint::ColPoint;
use super::conf::*;
use super::key::*;
use super::pieces::Piece;
use super::render::*;
use super::shadow::Shadow;
use super::update::*;
use super::util::*;

use piston_window::*;

pub struct State {
    pub mrate: u16,
    pub cl: u8,
    pub rate: u8,
    pub pieces: Vec<ColPoint>,
    pub p: Piece,
    pub shadow: Shadow,
    pub next: Piece,
    pub score: u32,
    pub window: PistonWindow,
}

// Driver code
impl State {
    fn create_widow() -> PistonWindow {
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
        window
    }

    pub fn new() -> Self {
        let p = random_piece();
        Self {
            //  gravity
            mrate: INIRATE,
            cl: 0,
            rate: 0,
            // Vector holding still pieces
            pieces: Vec::new(),
            p,
            shadow: Shadow::new(&p),
            next: random_piece(),
            score: 0,
            window: Self::create_widow(),
        }
    }
}
