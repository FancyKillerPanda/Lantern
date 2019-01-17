use lantern;
use lantern::application::Application;
use log;


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
    log::trace!("Logging works!");
    
    let sandbox_app = Sandbox::new();
    sandbox_app.run();
}
