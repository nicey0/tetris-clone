use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;
use piston_window::*;

#[derive(Debug)]
pub struct Game {
    pub rate: u16,
    pub mrate: u16,
    pub cl: u8,
    pub p: Piece,
    pub shadow: Shadow,
    pub next: Piece,
    pub pieces: Vec<ColPoint>,
    pub score: u32,
}

// Driver code
impl Game {
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
        }
    }

    pub fn mainloop(&mut self, window: &mut PistonWindow) {
        let mut font = window
            .load_font("fonts/FiraSans-Regular.ttf")
            .expect("error loading font");
        while let Some(e) = window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
                print!("\x1B[2J\x1B[1;1H");
                println!("{} / {}", self.rate, self.mrate);
                match self.update() {
                    States::End => break,
                    _ => {}
                };
            } else if let Some(_) = e.render_args() {
                // RENDER
                window.draw_2d(&e, |c, g, _d| {
                    clear(BACKGROUND, g);
                    self.draw_well(&c, g);
                    self.draw_next(&c, g);
                    self.draw_pieces(&c, g);
                });
                //for (i, ch) in self.score.to_string().chars().enumerate() {
                window.draw_2d(&e, |c, g, _d| self.draw_text(&c, g, "aaa", &mut font));
                window.draw_2d(&e, |c, g, _d| self.draw_text(&c, g, "bbb", &mut font));
                window.draw_2d(&e, |c, g, _d| self.draw_text(&c, g, "aaa", &mut font));
            } else if let Some(button) = e.press_args() {
                self.handle_button(button);
            }
        }
    }
}
//let mut font = Glyphs::from_bytes(
//include_bytes!("../fonts/FiraSans-Regular.ttf"),
//window.create_texture_context(),
//TextureSettings::new(),
//)
//.unwrap();
