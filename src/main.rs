mod colpoint;
mod conf;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod state;
mod update;
mod util;
use state::State;

fn main() {
    let game = State::new();
    game.mainloop();
}
