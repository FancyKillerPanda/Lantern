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
    log::trace!("Testing trace macro with colour.");
    log::info!("Testing info macro with colour.");
    log::warn!("Testing warn macro with colour.");
    log::error!("Testing error macro with colour.");
    log::fatal!("Testing fatal macro with colour.");
    log::trace!("Testing trace macro with one argument. Arg = {}", 42);
    
    let sandbox_app = Sandbox::new();
    sandbox_app.run();
}
