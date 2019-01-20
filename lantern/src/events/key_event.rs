use std::string::ToString;
use super::{ Event, EventType, EventCategory };

pub struct KeyPressedEvent {
	handled: bool,
	keycode: i32,
	repeat_count: i32,
}

impl KeyPressedEvent {
	pub fn new(keycode: i32, repeat_count: i32) -> Self {
		KeyPressedEvent {
			handled: false,
			keycode,
			repeat_count,
		}
	}

	pub fn get_repeat_count(&self) -> i32 {
		self.repeat_count
	}

	pub fn get_keycode(&self) -> i32 {
		self.keycode
	}
}

impl Event for KeyPressedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::KeyPressed
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Keyboard as i8) | (EventCategory::Input as i8)
	}
}

impl ToString for KeyPressedEvent {
	fn to_string(&self) -> String {
		String::from(format!("KeyPressedEvent: {} ({} repeats)", self.keycode, self.repeat_count))
	}
}


pub struct KeyReleasedEvent {
	handled: bool,
	keycode: i32,
}

impl KeyReleasedEvent {
	pub fn new(keycode: i32) -> Self {
		KeyReleasedEvent {
			handled: false,
			keycode,
		}
	}

	pub fn get_keycode(&self) -> i32 {
		self.keycode
	}
}

impl Event for KeyReleasedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::KeyReleased
	}

	fn get_handled(&self) -> bool {
		self.handled
	}

	fn set_handled(&mut self, state: bool) {
		self.handled = state;
	}

	fn get_category_flags(&self) -> i8 {
		(EventCategory::Keyboard as i8) | (EventCategory::Input as i8)
	}
}

impl ToString for KeyReleasedEvent {
	fn to_string(&self) -> String {
		String::from(format!("KeyReleasedEvent: {}", self.keycode))
	}
}