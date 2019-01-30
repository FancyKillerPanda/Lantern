use crate::window_base::Window;

/// Main engine entry point
/// 
/// Application should be implemented by the client's main struct.
pub trait Application {
	/// Initialises the application
	fn init(&mut self) {
		gl::load_with(|s| self.get_window().window.get_proc_address(s));
	}

	/// Runs the engine
	fn run(&mut self) {
		// TEMPORARY CODE //
		
		unsafe { gl::ClearColor(1.0, 0.0, 1.0, 1.0) };
		
		while self.get_running() {
			unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
			self.get_window().on_update();
		}
	}

	/// Returns true if the application is still running
	fn get_running(&self) -> bool;

	/// Returns the application's window instance
	fn get_window(&mut self) -> &mut Window;
}
