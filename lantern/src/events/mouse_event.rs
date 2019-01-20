use std::string::ToString;
use super::{ Event, EventType, EventCategory, Position, Offset };

pub struct MouseMovedEvent {
	handled: bool,
	position: Position,
}

impl MouseMovedEvent {
	pub fn new(position: Position) -> Self {
		MouseMovedEvent {
			handled: false,
			position,
		}
	}

	pub fn get_position(&self) -> &Position {
		&self.position
	}
}

impl Event for MouseMovedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseMoved(self.position)
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Mouse as i8) | (EventCategory::Input as i8)
	}
}

impl ToString for MouseMovedEvent {
	fn to_string(&self) -> String {
		String::from(format!("MouseMovedEvent: ({}, {})", self.position.0, self.position.1))
	}
}


pub struct MouseScrolledEvent {
	handled: bool,
	offset: Offset,
}

impl MouseScrolledEvent {
	pub fn new(offset: Offset) -> Self {
		MouseScrolledEvent {
			handled: false,
			offset,
		}
	}

	pub fn get_offset(&self) -> &Offset {
		&self.offset
	}
}

impl Event for MouseScrolledEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseScrolled(self.offset)
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Mouse as i8) | (EventCategory::Input as i8)
	}
}

impl ToString for MouseScrolledEvent {
	fn to_string(&self) -> String {
		String::from(format!("MouseScrolledEvent: ({}, {})", self.offset.0, self.offset.1))
	}
}


pub struct MouseButtonPressedEvent {
	handled: bool,
	button: i32,
}

impl MouseButtonPressedEvent {
	pub fn new(button: i32) -> Self {
		MouseButtonPressedEvent {
			handled: false,
			button,
		}
	}

	pub fn get_mouse_button(&self) -> i32 {
		self.button
	}
}

impl Event for MouseButtonPressedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseButtonPressed
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Mouse as i8) | (EventCategory::Input as i8)
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for MouseButtonPressedEvent {
	fn to_string(&self) -> String {
		String::from(format!("MouseButtonPressedEvent: {}", self.button))
	}
}


pub struct MouseButtonReleasedEvent {
	handled: bool,
	button: i32,
}

impl MouseButtonReleasedEvent {
	pub fn new(button: i32) -> Self {
		MouseButtonReleasedEvent {
			handled: false,
			button,
		}
	}

	pub fn get_mouse_button(&self) -> i32 {
		self.button
	}
}

impl Event for MouseButtonReleasedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseButtonReleased
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Mouse as i8) | (EventCategory::Input as i8)
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}
}

impl ToString for MouseButtonReleasedEvent {
	fn to_string(&self) -> String {
		String::from(format!("MouseButtonReleasedEvent: {}", self.button))
	}
}
