use crate::Size;

#[cfg_attr(windows, path = "platform/windows/windows_window.rs")]
pub mod window;
pub use window::*;

/// The properties of a window
/// 
/// It uses the builder pattern with default values
pub struct WindowProps {
	/// The title of the window
	pub title: &'static str,
	/// The size of the window
	pub size: Size,
}

impl WindowProps {
	/// Creates a new properties instance
	pub fn new() -> Self {
		WindowProps {
			title: "Lantern Engine!",
			size: Size(960, 540),
		}
	}

	/// Sets the title of the properties
	pub fn title(mut self, t: &'static str) -> Self {
		self.title = t;
		self
	}

	/// Sets the size of the window
	pub fn size(mut self, s: Size) -> Self {
		self.size = s;
		self
	}
}

impl Default for WindowProps {
	fn default() -> Self {
		Self::new()
	}
}
