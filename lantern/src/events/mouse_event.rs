use std::string::ToString;
use super::{ Event, EventType, EventCategory, Position, Offset };

/// Event for when the mouse moves.String
/// 
/// Position is the new position of the mouse.
pub struct MouseMovedEvent {
	handled: bool,
	position: Position,
}

impl MouseMovedEvent {
	/// Creates a new MouseMovedEvent.	
	pub fn new(position: Position) -> Self {
		MouseMovedEvent {
			handled: false,
			position,
		}
	}

	/// Gets the position of the mouse in this event.
	pub fn get_position(&self) -> &Position {
		&self.position
	}
}

impl Event for MouseMovedEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseMoved
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
		format!("MouseMovedEvent: ({}, {})", self.position.0, self.position.1)
	}
}


/// Event for when the mouse wheel is scrolled.
/// 
/// Offset is how far / in what direction it was scrolled.
pub struct MouseScrolledEvent {
	handled: bool,
	offset: Offset,
}

impl MouseScrolledEvent {
	/// Creates a new MouseScrolledEvent.
	pub fn new(offset: Offset) -> Self {
		MouseScrolledEvent {
			handled: false,
			offset,
		}
	}

	/// Gets the offset of the scroll wheel
	pub fn get_offset(&self) -> &Offset {
		&self.offset
	}
}

impl Event for MouseScrolledEvent {
	fn get_event_type(&self) -> EventType {
		EventType::MouseScrolled
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
		format!("MouseScrolledEvent: ({}, {})", self.offset.0, self.offset.1)
	}
}


/// Event for when a mouse button is pressed.
pub struct MouseButtonPressedEvent {
	handled: bool,
	button: i32,
}

impl MouseButtonPressedEvent {
	/// Creates a new MouseButtonPressedEvent.	
	pub fn new(button: i32) -> Self {
		MouseButtonPressedEvent {
			handled: false,
			button,
		}
	}

	/// Gets the button that was pressed.
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
		format!("MouseButtonPressedEvent: {}", self.button)
	}
}


/// Event for when a mouse button is released.
pub struct MouseButtonReleasedEvent {
	handled: bool,
	button: i32,
}

impl MouseButtonReleasedEvent {
	/// Creates a new MouseButtonReleasedEvent.	
	pub fn new(button: i32) -> Self {
		MouseButtonReleasedEvent {
			handled: false,
			button,
		}
	}

	/// Gets the button that was pressed.
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
		format!("MouseButtonReleasedEvent: {}", self.button)
	}
}
