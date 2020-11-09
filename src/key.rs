use super::game::Game;
use super::util::*;
use piston_window::{Button, Key};

impl Game {
    pub fn handle_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => {
                self.handle_key(key);
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: Key) -> States {
        match key {
            Key::Right => {
                self.p.side(1, &self.pieces);
            }
            Key::Left => {
                self.p.side(-1, &self.pieces);
            }
            Key::Up => self.p.rotate(&self.pieces),
            Key::Down => {
                self.rate = self.mrate;
            }
            Key::Space => {
                self.rate = self.mrate;
                self.p.put_down(&self.pieces)
            }
            Key::P => return States::Pause,
            _ => {}
        };
        States::Nothing
    }
}
