use super::state::State;
use piston_window::*;

impl State {
    pub fn mainloop(&mut self, window: &mut PistonWindow) {
        let mut font = window
            .load_font("fonts/FiraSans-Regular.ttf")
            .expect("error loading font");
        while let Some(e) = window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
                print!("\x1B[2J\x1B[1;1H");
                //println!("{:#?}", self);
                println!("{} / {}", self.rate, self.mrate);
                println!("{}", self.cl);
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
                for (i, ch) in "aaabbb".chars().enumerate() {
                    window.draw_2d(&e, |c, g, _d| self.draw_letter(&c, g, i, ch, &mut font));
                }
            } else if let Some(button) = e.press_args() {
                self.handle_button(button);
            }
        }
    }
}
