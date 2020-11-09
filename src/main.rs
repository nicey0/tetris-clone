mod colpoint;
mod conf;
mod driver;
mod key;
mod piece_defs;
mod pieces;
mod render;
mod shadow;
mod state;
mod update;
mod util;
use driver::*;
use state::State;

fn main() {
    let game = State::new();
    game.mainloop();
}
