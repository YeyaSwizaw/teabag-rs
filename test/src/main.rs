extern crate teabag;

use teabag::game::{Game, State};
use teabag::signal::Slot;
use teabag::keyboard::{Key, KeyCode};

use std::rc::Rc;
use std::cell::RefCell;

fn quit(state: &mut State) {
    state.terminate();
    println!(":o");
}

fn main() {
    let x = Rc::new(RefCell::new(0u64));

    let x_ref_a = x.clone();
    let x_ref_b = x.clone();
    let local = x.clone();

    let game = Game::new();
    game.with_signals(|signals| {
        signals.close.connect(quit);

        signals.key_release.connect(move |state: &mut State, key: Key| {
            println!("{:?}", key);

            let mut x = x_ref_a.borrow_mut();
            *x -= (key.scan_code / 2) as u64;
            println!("Total: {}", *x);

            if let KeyCode::Escape = key.key {
                quit(state);
            }
        });

        signals.key_press.connect(move |state: &mut State, key: Key| {
            println!("{:?}", key);

            let mut x = x_ref_b.borrow_mut();
            *x += key.scan_code as u64;
            println!("Total: {}", *x);

            if let KeyCode::Escape = key.key {
                quit(state);
            }
        });
    });

    game.run();

    let x = local.borrow();
    println!("Final Total: {}", *x);
}
