use std::fmt;

pub mod application_event;
pub use application_event::*;
pub mod key_event;
pub use key_event::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position(pub u32, pub u32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Offset(pub u32, pub u32);


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EventType {
	None,
	WindowClose, WindowResize(Size), WindowFocus, WindowLostFocus, WindowMoved(Position),
	AppTick, AppUpdate, AppRender,
	KeyPressed, KeyReleased,
	MouseButtonPressed, MouseButtonReleased, MouseMoved(Position), MouseScrolled(Offset),
}

impl fmt::Display for EventType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}


#[derive(Clone, Copy, PartialEq)]
pub enum EventCategory {
	None,
	Application = 1 << 0,
	Input = 1 << 1,
	Keyboard = 1 << 2,
	Mouse = 1 << 3,
	MouseButton = 1 << 4,
}


pub trait Event {
	fn get_event_type(&self) -> EventType;
	fn get_category_flags(&self) -> i8;

	fn get_handled(&self) -> bool;
	fn set_handled(&mut self, state: bool);

	fn get_name(&self) -> String {
		let mut s = String::from(self.get_event_type().to_string());
		s.push_str("Event");
		s
	}

	fn is_in_category(&self, category: EventCategory) -> bool {
		(self.get_category_flags() & (category as i8)) != 0
	}
}


pub struct EventDispatcher {
	event: Box<dyn Event>,
}

impl EventDispatcher {
	pub fn new(event: Box<dyn Event>) -> Self {
		EventDispatcher {
			event,
		}
	}

	pub fn dispatch(&mut self, event: Box<dyn Event>, func: fn(&mut Box<dyn Event>) -> bool) -> bool {
		if self.event.get_event_type() == event.get_event_type() {
			let result = func(&mut self.event);
			self.event.set_handled(result);
			return true;
		}

		false
	}
}
