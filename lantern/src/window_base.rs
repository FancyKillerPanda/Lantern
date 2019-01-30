use crate::Size;

#[cfg_attr(windows, path = "platform/windows/windows_window.rs")]
pub mod window;
pub use window::*;

pub struct WindowProps {
	pub title: &'static str,
	pub size: Size,
}

impl WindowProps {
	pub fn new_def() -> Self {
		WindowProps::new("Lantern Engine!", Size(960, 540))
	}

	pub fn new(title: &'static str, size: Size) -> Self {
		WindowProps {
			title,
			size,
		}
	}
}
