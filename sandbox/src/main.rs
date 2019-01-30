use lantern;
use lantern::application::Application;
use lantern::window_base::{ Window, WindowProps };

struct Sandbox {
    running: bool,
    window: Window,
}

impl Sandbox {
    fn new() -> Self {
        let mut s = Sandbox {
            running: true,
            window: Window::new(WindowProps::new()),
        };

        s.init();
        s
    }
}

impl Application for Sandbox {
    fn get_running(&self) -> bool {
        self.running
    }

    fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }
}


fn main() {
    let mut sandbox_app = Sandbox::new();
    sandbox_app.run();
}
