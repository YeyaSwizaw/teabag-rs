extern crate teabag;

use teabag::{Game, State};
use teabag::signal::Slot;
use teabag::keyboard::{Key, KeyCode};

fn quit(state: &mut State) {
    state.terminate();
    println!(":o");
}

fn main() {
    let mut game = Game::new();

    game.signals.close.connect(quit);

    game.signals.key_press.connect(|state: &mut State, key: Key| {
        println!("{:?}", key);

        if let KeyCode::Escape = key.key {
            quit(state);
        }
    });

    game.run();
}
