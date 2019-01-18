enum EventType {
	None,
	WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
	AppTick, AppUpdate, AppRender,
	KeyPressed, KeyReleased,
	MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled,
}

enum EventCategory {
	None,
	Application = 1 << 0,
	Input = 1 << 1,
	Keyboard = 1 << 2,
	Mouse = 1 << 3,
	MouseButton = 1 << 4,
}

trait Event {
	fn get_event_type(&self) -> EventType;
	fn get_name(&self) -> &String;
	fn get_category_flags(&self) -> i8;

	fn to_string(&self) -> &String {
		self.get_name()
	}

	fn is_in_category(&self, category: EventCategory) -> bool {
		(self.get_category_flags() & (category as i8)) != 0
	}
}

struct EventDispatcher<'a> {
	event: &'a mut Event,
}

impl<'a> EventDispatcher<'a> {
	pub fn new(event: &'a mut Event) -> Self {
		EventDispatcher {
			event,
		}
	}

	pub fn dispatch<T>(func: fn(&mut T) -> bool) -> bool {
		if (self.event.get_event_type() == )
	}
}
