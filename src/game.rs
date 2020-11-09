use super::colpoint::ColPoint;
use super::conf::*;
use super::pieces::Piece;
use super::shadow::Shadow;
use super::util::*;
use piston_window::*;

#[derive(Debug)]
pub struct Game {
    pub rate: u16,             // gravity
    pub mrate: u16,            // rate == mrate: do_gravity
    pub level: u16,            // current level
    pub cl: u8,                // cleared line counter
    pub p: Piece,              // current piece
    pub shadow: Shadow,        // current pieces shadow (thingy at the bottom)
    pub next: Piece,           // next piece
    pub pieces: Vec<ColPoint>, // placed blocks
    pub score: u32,            // current score
}

// Driver code
impl Game {
    pub fn new() -> Self {
        let p = random_piece();
        Self {
            //  gravity
            rate: 0,
            mrate: INIRATE,
            level: 0,
            cl: 0,
            p,
            shadow: Shadow::new(&p),
            next: random_piece(),
            pieces: Vec::new(),
            score: 0,
        }
    }

    pub fn mainloop(&mut self, window: &mut PistonWindow) {
        let mut glyphs = window
            .load_font("fonts/FiraSans-Regular.ttf")
            .expect("error loading font!");
        while let Some(e) = window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
                print!("\x1B[2J\x1B[1;1H"); // clear screen
                println!("{} / {}", self.rate, self.mrate);
                match self.update() {
                    States::End => break,
                    _ => {}
                };
            } else if let Some(_) = e.render_args() {
                // RENDER
                window.draw_2d(&e, |c, g, _d| {
                    clear(WELL, g);
                    self.draw_right(&c, g);
                    self.draw_next(&c, g);
                    self.draw_pieces(&c, g);
                });
                window.draw_2d(&e, |c, g, dev| {
                    self.draw_text(
                        &c,
                        g,
                        &format!("Score: {}", self.score),
                        &mut glyphs,
                        0.0,
                        0.0,
                    );
                    self.draw_text(
                        &c,
                        g,
                        &format!("Level: {}", self.level),
                        &mut glyphs,
                        0.0,
                        CELLSIZE,
                    );
                    self.draw_text(
                        &c,
                        g,
                        &format!("Clears: {}", CLEARS - self.cl),
                        &mut glyphs,
                        0.0,
                        CELLSIZE * 2.0,
                    );
                    glyphs.factory.encoder.flush(dev);
                });
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
