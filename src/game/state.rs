pub struct State {
    should_quit: bool
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

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}
