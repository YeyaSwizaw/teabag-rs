use ::signal::{Signal0, Signal1};
use ::keyboard::Key;
use ::game::State;

#[derive(Default)]
pub struct GameSignals {
    pub close: Signal0<State>,
    pub key_press: Signal1<State, Key>,
    pub key_release: Signal1<State, Key>
}
