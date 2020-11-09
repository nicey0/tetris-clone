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
use game::State;
use piston_window::*;
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

impl State {
    pub fn mainloop(&mut self, window: &mut PistonWindow) {
        //let mut font = window
        //.load_font("fonts/FiraSans-Regular.ttf")
        //.expect("error loading font");
        let mut font = Glyphs::from_bytes(
            include_bytes!("../fonts/FiraSans-Regular.ttf"),
            window.create_texture_context(),
            TextureSettings::new(),
        )
        .unwrap();
        while let Some(e) = window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
                print!("\x1B[2J\x1B[1;1H");
                println!("{} / {}", self.rate, self.mrate);
                if !self.update() {
                    break;
                }
            } else if let Some(_) = e.render_args() {
                // RENDER
                window.draw_2d(&e, |c, g, _d| {
                    clear([0.1, 0.1, 0.1, 1.0], g);
                    self.draw_well(&c, g);
                    self.draw_next(&c, g);
                    self.draw_pieces(&c, g);
                });
                //for (i, ch) in self.score.to_string().chars().enumerate() {
                window.draw_2d(&e, |c, g, _d| self.draw_letter(&c, g, "aaabbb", &mut font));
            } else if let Some(button) = e.press_args() {
                self.handle_button(button);
            }
        }
    }
}
