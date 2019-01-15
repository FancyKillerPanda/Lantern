use lantern;
use lantern::application::Application;


struct Sandbox {
}

impl Sandbox {
    fn new() -> Self {
        Sandbox {}
    }
}

impl Application for Sandbox {
}


fn main() {
    let sandbox_app = Sandbox::new();
    sandbox_app.run();
}
