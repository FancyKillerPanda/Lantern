use std::string::ToString;
use super::{ Event, EventType, EventCategory };

/// Event for a key press.
/// 
/// Keycode is the key that was pressed.
/// Repeat count is how many times the key repeated.
pub struct KeyPressedEvent {
	handled: bool,
	keycode: i32,
	repeat_count: i32,
}

impl KeyPressedEvent {
	/// Creates a new KeyPressedEvent.
	pub fn new(keycode: i32, repeat_count: i32) -> Self {
		KeyPressedEvent {
			handled: false,
			keycode,
			repeat_count,
		}
	}

	/// Returns the repeat count of the event.
	pub fn get_repeat_count(&self) -> i32 {
		self.repeat_count
	}

	/// Returns the keycode of the event.
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
		format!("KeyPressedEvent: {} ({} repeats)", self.keycode, self.repeat_count)
	}
}


/// Event for a key release.
/// 
/// Keycode is the key that was pressed.
pub struct KeyReleasedEvent {
	handled: bool,
	keycode: i32,
}

impl KeyReleasedEvent {
	/// Creates a new KeyReleasedEvent.
	pub fn new(keycode: i32) -> Self {
		KeyReleasedEvent {
			handled: false,
			keycode,
		}
	}

	/// Gets the keycode of the event.
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
		format!("KeyReleasedEvent: {}", self.keycode)
	}
}
