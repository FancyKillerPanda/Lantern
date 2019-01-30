use std::sync::mpsc;
use std::process;
use glfw::Context;
use crate::Size;
use crate::events::Event;
use crate::window_base::WindowProps;

type EventCallbackFn = fn(&mut Box<dyn Event>);

struct WindowData {
	title: &'static str,
	size: Size,
	vsync: bool,
	event_callback: Option<EventCallbackFn>,
}

pub struct Window {
	pub glfw: glfw::Glfw,
	pub window: glfw::Window,
	events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
	data: WindowData,
}

impl Window {
	pub fn new(props: WindowProps) -> Self {
		let glfw_result = match glfw::init(glfw::FAIL_ON_ERRORS) {
				Ok(g) => g,
				Err(msg) => {
					log::core_fatal!("Could not initialise GLFW. {}", msg);
					process::exit(1);
				},
		};
		
		let (glfw_window, glfw_events) = match glfw_result.create_window(props.size.0, props.size.1, props.title, glfw::WindowMode::Windowed) {
			Some((w, e)) => (w, e),
			None => {
				log::core_fatal!("Could not create GLFW window and event handler.");
				process::exit(1);
			}
		};

		let mut win = Window {
			glfw: glfw_result,
			window: glfw_window,
			events: glfw_events,
			data: WindowData {
				title: props.title,
				size: props.size,
				vsync: true,
				event_callback: None,
			}
		};

		log::core_info!("Created window {} ({}, {})", win.data.title, win.data.size.0, win.data.size.1);

		win.window.make_current();
		win.set_vsync(true);

		win
	}

	pub fn on_update(&mut self) {
		self.glfw.poll_events();
		self.window.swap_buffers();
	}

	pub fn get_size(&self) -> &Size {
		&self.data.size
	}

	pub fn set_event_callback(&mut self, callback: EventCallbackFn) {
		self.data.event_callback = Some(callback);
	}

	pub fn set_vsync(&mut self, enabled: bool) {
		if enabled {
			self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
		} else {
			self.glfw.set_swap_interval(glfw::SwapInterval::None);
		}

		self.data.vsync = enabled;
	}

	pub fn get_vsync(&self) -> bool {
		self.data.vsync
	}
}
