use crate::Size;

#[cfg_attr(windows, path = "platform/windows/windows_window.rs")]
pub mod window;
pub use window::*;

pub struct WindowProps {
	pub title: &'static str,
	pub size: Size,
}

impl WindowProps {
	pub fn new() -> Self {
		WindowProps {
			title: "Lantern Engine!",
			size: Size(960, 540),
		}
	}

	pub fn title(mut self, t: &'static str) -> Self {
		self.title = t;
		self
	}

	pub fn size(mut self, s: Size) -> Self {
		self.size = s;
		self
	}
}
