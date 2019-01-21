use std::fmt;

pub mod application_event;
pub use application_event::*;
pub mod key_event;
pub use key_event::*;
pub mod mouse_event;
pub use mouse_event::*;

/// Represents a ize
/// 
/// The two fields are a `width` and a `height`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

/// Represents a position
/// 
/// The two fields are an `x` and a `y`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position(pub u32, pub u32);

/// Represents an offset
/// 
/// The two fields are an `xOffset` and a `yOffset`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Offset(pub u32, pub u32);


/// Represents a type of Event
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EventType {
	None,
	WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
	AppTick, AppUpdate, AppRender,
	KeyPressed, KeyReleased,
	MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled,
}

impl fmt::Display for EventType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Prints out the variant
		write!(f, "{:?}", self)
	}
}


/// Represents what category an Event is in.
/// 
/// An Event can be in multiple categories. The values
/// of the variants are bit-shifted for easier access
/// using bitwise and and bitwise or.
#[derive(Clone, Copy, PartialEq)]
pub enum EventCategory {
	None,
	Application = 1 << 0,
	Input = 1 << 1,
	Keyboard = 1 << 2,
	Mouse = 1 << 3,
	MouseButton = 1 << 4,
}


/// Represents an event in the engine.
pub trait Event {
	/// Gets the type of the event.
	fn get_event_type(&self) -> EventType;
	/// Gets which categories the event is in
	/// 
	/// Represented as an `i8` with bit-shifted
	/// values. Can be more than one category.
	fn get_category_flags(&self) -> i8;

	/// Gets whether the event has been handled
	fn get_handled(&self) -> bool;
	/// Sets whether the event has been handled
	fn set_handled(&mut self, state: bool);

	/// Gets the name of the event
	fn get_name(&self) -> String {
		let mut s = String::from(self.get_event_type().to_string());
		s.push_str("Event");
		s
	}

	/// Checks if a given event is in a categoy
	fn is_in_category(&self, category: EventCategory) -> bool {
		(self.get_category_flags() & (category as i8)) != 0
	}
}


/// Struct that dispatches an event
pub struct EventDispatcher {
	event: Box<dyn Event>,  // Something that implements the Event trait
}

impl EventDispatcher {
	/// Creates a new EventDispatcher
	pub fn new(event: Box<dyn Event>) -> Self {
		EventDispatcher {
			event,
		}
	}

	/// Dispatches an event
	pub fn dispatch(&mut self, event: Box<dyn Event>, func: fn(&mut Box<dyn Event>) -> bool) -> bool {
		if self.event.get_event_type() == event.get_event_type() {
			let result = func(&mut self.event);  // Gets the result of the function (bool)
			self.event.set_handled(result);
			return true;
		}

		false
	}
}
