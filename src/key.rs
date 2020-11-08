use super::colpoint::ColPoint;
use super::pieces::Piece;
use super::util::*;
use piston::{Button, Key};

pub fn handle_button(button: Button, p: &mut Piece, pieces: &Vec<ColPoint>) {
    match button {
        Button::Keyboard(key) => {
            handle_key(key, p, &pieces);
        }
        _ => {}
    }
}

fn handle_key(key: Key, p: &mut Piece, pieces: &Vec<ColPoint>) -> States {
    match key {
        Key::Right => p.side(1, &pieces),
        Key::Left => p.side(-1, &pieces),
        Key::Up => p.rotate(&pieces),
        Key::Down => p.put_down(&pieces),
        Key::P => return States::Pause,
        _ => {}
    };
    States::Nothing
}
