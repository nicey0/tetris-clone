use super::state::State;
use piston_window::*;

impl State {
    pub fn mainloop(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            if let Some(_) = e.update_args() {
                // UPDATE
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
                for (i, ch) in self.score.to_string().chars().enumerate() {
                    let mut font = match window.load_font("fonts/FiraSans-Regular.ttf") {
                        Ok(f) => f,
                        Err(e) => panic!("{}", e),
                    };
                    window.draw_2d(&e, |c, g, _d| self.draw_letter(&c, g, i, ch, &mut font));
                }
            } else if let Some(button) = e.press_args() {
                self.handle_button(button);
            }
        }
    }
}
