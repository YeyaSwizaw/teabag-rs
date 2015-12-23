extern crate gl;
extern crate glutin;
extern crate libc;

use signal::{Signal, Signal0, Signal1};
use keyboard::Key;

use std::rc::Rc;
use std::cell::RefCell;

pub mod signal;
pub mod keyboard;

pub struct State {
    should_quit: bool
}

#[derive(Default)]
pub struct GameSignals {
    pub close: Signal0<State>,
    pub key_press: Signal1<State, Key>,
    pub key_release: Signal1<State, Key>
}

pub struct Game {
    state: Rc<RefCell<State>>,
    pub signals: GameSignals
}

impl State {
    pub fn new() -> State {
        State {
            should_quit: false
        }
    }

    pub fn terminate(&mut self) {
        self.should_quit = true;
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            signals: Default::default(),
            state: Rc::new(RefCell::new(State::new()))
        }
    }

    pub fn run(self) {
        let window = glutin::Window::new().unwrap();

        unsafe {
            window.make_current();

            gl::load_with(|symbol| window.get_proc_address(symbol) as *const std::os::raw::c_void);
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        }

        let state = self.state.clone();

        for event in window.wait_events() {
            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            window.swap_buffers();

            match event {
                glutin::Event::Closed => self.signals.close.fire(state.clone(), ()),

                glutin::Event::KeyboardInput(key_state, scan_code, key_code) => {
                    let code = Key::new(scan_code, key_code);
                    if let glutin::ElementState::Pressed = key_state {
                        self.signals.key_press.fire(state.clone(), (code,))
                    } else {
                        self.signals.key_release.fire(state.clone(), (code,))
                    }
                },

                _ => ()
            }

            if state.borrow().should_quit {
                break
            }
        }
    }
}
