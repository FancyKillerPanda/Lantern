use std::string::ToString;
use super::{ Event, EventType, EventCategory, Size };

pub struct WindowResizeEvent {
	handled: bool,
	size: Size,
}

impl WindowResizeEvent {
	pub fn new(size: Size) -> Self {
		WindowResizeEvent {
			handled: false,
			size,
		}
	}

	pub fn get_size(&self) -> Size {
		self.size
	}
}

impl Event for WindowResizeEvent {
	fn get_event_type(&self) -> EventType {
		EventType::WindowResize(self.size)
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
		String::from(format!("WindowResizeEvent: ({}, {})", self.size.0, self.size.1))
	}
}


pub struct WindowClosedEvent {
	handled: bool,
}

impl WindowClosedEvent {
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
		String::from(self.get_name())
	}
}


pub struct AppTickEvent {
	handled: bool,
}

impl AppTickEvent {
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
		String::from(self.get_name())
	}
}


pub struct AppUpdateEvent {
	handled: bool,
}

impl AppUpdateEvent {
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
		String::from(self.get_name())
	}
}


pub struct AppRenderEvent {
	handled: bool,
}

impl AppRenderEvent {
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
		String::from(self.get_name())
	}
}
