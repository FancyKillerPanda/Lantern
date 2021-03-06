use std::string::ToString;
use super::{ Event, EventType, EventCategory };
use crate::Size;

/// Event when the window is resized.
/// 
/// Holds the new size of the window.
pub struct WindowResizeEvent {
	handled: bool,
	size: Size,
}

impl WindowResizeEvent {
	/// Creates a new WindowResizeEvent.
	pub fn new(size: Size) -> Self {
		WindowResizeEvent {
			handled: false,
			size,
		}
	}

	/// Returns the new size of the window.
	pub fn get_size(&self) -> Size {
		self.size
	}
}

impl Event for WindowResizeEvent {
	fn get_event_type(&self) -> EventType {
		EventType::WindowResize
	}

	fn get_category_flags(&self) -> i8 {
		EventCategory::Application as i8
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for WindowResizeEvent {
	fn to_string(&self) -> String {
		format!("WindowResizeEvent: ({}, {})", self.size.0, self.size.1)
	}
}


/// Event when the window is closed.
#[derive(Default)]
pub struct WindowClosedEvent {
	handled: bool,
}

impl WindowClosedEvent {
	/// Creates a new WindowClosedEvent.
	pub fn new() -> Self {
		WindowClosedEvent {
			handled: false,
		}
	}
}

impl Event for WindowClosedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::WindowClose
	}

	fn get_category_flags(&self) -> i8 {
		EventCategory::Application as i8
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for WindowClosedEvent {
	fn to_string(&self) -> String {
		self.get_name()
	}
}


/// Event for when the app ticks.
#[derive(Default)]
pub struct AppTickEvent {
	handled: bool,
}

impl AppTickEvent {
	/// Creates a new AppTickEvent.
	pub fn new() -> Self {
		AppTickEvent {
			handled: false,
		}
	}
}

impl Event for AppTickEvent {
	fn get_event_type(&self) -> EventType {
		EventType::AppTick
	}

	fn get_category_flags(&self) -> i8 {
		EventCategory::Application as i8
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for AppTickEvent {
	fn to_string(&self) -> String {
		self.get_name()
	}
}


/// Event for when the app updates.
#[derive(Default)]
pub struct AppUpdateEvent {
	handled: bool,
}

impl AppUpdateEvent {
	/// Creates a new AppUpdateEvent.
	pub fn new() -> Self {
		AppUpdateEvent {
			handled: false,
		}
	}
}

impl Event for AppUpdateEvent {
	fn get_event_type(&self) -> EventType {
		EventType::AppUpdate
	}

	fn get_category_flags(&self) -> i8 {
		EventCategory::Application as i8
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for AppUpdateEvent {
	fn to_string(&self) -> String {
		self.get_name()
	}
}


/// Event for when rendering occurs.
#[derive(Default)]
pub struct AppRenderEvent {
	handled: bool,
}

impl AppRenderEvent {
	/// Creates a new AppRenderEvent.
	pub fn new() -> Self {
		AppRenderEvent {
			handled: false,
		}
	}
}

impl Event for AppRenderEvent {
	fn get_event_type(&self) -> EventType {
		EventType::AppRender
	}

	fn get_category_flags(&self) -> i8 {
		EventCategory::Application as i8
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for AppRenderEvent {
	fn to_string(&self) -> String {
		self.get_name()
	}
}
