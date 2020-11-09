use super::colpoint::ColPoint;
use super::pieces::Piece;
use super::util::*;
use piston_window::{Button, Key};

pub fn handle_button(
    button: Button,
    p: &mut Piece,
    pieces: &Vec<ColPoint>,
    rate: &mut u16,
    mrate: &u16,
) {
    match button {
        Button::Keyboard(key) => {
            handle_key(key, p, &pieces, rate, mrate);
        }
        _ => {}
    }
}

fn handle_key(
    key: Key,
    p: &mut Piece,
    pieces: &Vec<ColPoint>,
    rate: &mut u16,
    &mrate: &u16,
) -> States {
    match key {
        Key::Right => p.side(1, &pieces),
        Key::Left => p.side(-1, &pieces),
        Key::Up => p.rotate(&pieces),
        Key::Down => {
            *rate = mrate;
        }
        Key::Space => {
            *rate = mrate;
            p.put_down(&pieces)
        }
        Key::P => return States::Pause,
        _ => {}
    };
    States::Nothing
}
