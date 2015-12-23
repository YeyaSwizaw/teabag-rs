use ::game::State;
use ::game::signal::GameSignals;
use ::signal::Signal;
use ::keyboard::Key;

use std::os;
use std::rc::Rc;
use std::cell::RefCell;

use ::glutin;
use ::gl;

pub struct Game {
    state: Rc<RefCell<State>>,
    signals: Rc<RefCell<GameSignals>>
}

impl Game {
    pub fn new() -> Game {
        Game {
            signals: Rc::new(RefCell::new(Default::default())),
            state: Rc::new(RefCell::new(State::new()))
        }
    }

    pub fn construct_new<F: FnOnce(&mut Game)>(constructor: F) -> Game {
        let mut game = Game::new();
        constructor(&mut game);
        game
    }

    pub fn with_signals<F: FnOnce(&mut GameSignals)>(&self, f: F) {
        f(&mut self.signals.borrow_mut())
    }

    pub fn run(self) {
        let window = glutin::Window::new().unwrap();

        unsafe {
            window.make_current();

            gl::load_with(|symbol| window.get_proc_address(symbol) as *const os::raw::c_void);
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        }

        let state = self.state.clone();
        let mut signals = self.signals.borrow_mut();

        for event in window.wait_events() {
            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            window.swap_buffers();

            match event {
                glutin::Event::Closed => signals.close.fire(state.clone(), ()),

                glutin::Event::KeyboardInput(key_state, scan_code, key_code) => {
                    let code = Key::new(scan_code, key_code);
                    if let glutin::ElementState::Pressed = key_state {
                        signals.key_press.fire(state.clone(), (code,))
                    } else {
                        signals.key_release.fire(state.clone(), (code,))
                    }
                },

                _ => ()
            }

            if state.borrow().should_quit() {
                break
            }
        }
    }
}
