use super::key::*;
use super::state::State;
use piston_window::*;

impl State {
    pub fn mainloop(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
                if !self.update() {
                    break;
                }
            } else if let Some(_) = e.render_args() {
                // RENDER
                self.window.draw_2d(&e, |c, g, _d| {
                    clear([0.1, 0.1, 0.1, 1.0], g);
                    self.draw_well(&c, g);
                    self.draw_next(&c, g);
                    self.draw_pieces(&c, g);
                });
                for (i, ch) in self.score.to_string().chars().enumerate() {
                    let mut font = match self.window.load_font("fonts/FiraSans-Regular.ttf") {
                        Ok(f) => f,
                        Err(e) => panic!("{}", e),
                    };
                    self.window
                        .draw_2d(&e, |c, g, _d| self.draw_letter(&c, g, i, ch, &mut font));
                }
            } else if let Some(button) = e.press_args() {
                handle_button(button, &mut p, &pieces, &mut rate, &mrate);
            }
        }
    }
}
