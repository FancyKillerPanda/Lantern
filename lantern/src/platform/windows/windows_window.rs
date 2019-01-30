use std::sync::mpsc;
use std::process;
use glfw::Context;
use crate::Size;
use crate::events::Event;
use crate::window_base::WindowProps;

type EventCallbackFn = fn(&mut Box<dyn Event>);

/// Represents the data of a window
struct WindowData {
	/// The title of the window
	title: &'static str,
	/// The size of the window
	size: Size,
	/// Whether VSync is enabled
	vsync: bool,
	/// An event callback for the window
	event_callback: Option<EventCallbackFn>,
}

/// Represents a Window
pub struct Window {
	/// Supplies the window with GLFW function calls
	pub glfw: glfw::Glfw,
	/// The GLFW window instance
	pub window: glfw::Window,
	/// The event handler for the instance
	events: mpsc::Receiver<(f64, glfw::WindowEvent)>,
	/// The data of the window
	data: WindowData,
}

impl Window {
	/// Creates a new window instance
	pub fn new(props: WindowProps) -> Self {
		// Initialises GLFW
		let glfw_result = match glfw::init(glfw::FAIL_ON_ERRORS) {
				Ok(g) => g,
				Err(msg) => {
					log::core_fatal!("Could not initialise GLFW. {}", msg);
					process::exit(1);
				},
		};
		
		// Creates the GLFW window instance
		let (glfw_window, glfw_events) = match glfw_result.create_window(props.size.0, props.size.1, props.title, glfw::WindowMode::Windowed) {
			Some((w, e)) => (w, e),
			None => {
				log::core_fatal!("Could not create GLFW window and event handler.");
				process::exit(1);
			}
		};

		// Creates the window
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

		// Logs creation of the window
		log::core_info!("Created window {} ({}, {})", win.data.title, win.data.size.0, win.data.size.1);

		// Makes the OpenGL context current
		win.window.make_current();
		// Turns on VSync for the window
		win.set_vsync(true);

		win
	}

	/// Updates the window
	pub fn on_update(&mut self) {
		self.glfw.poll_events();
		self.window.swap_buffers();
	}

	/// Returns the size of the window
	pub fn get_size(&self) -> &Size {
		&self.data.size
	}

	/// Sets an event callback for the window
	pub fn set_event_callback(&mut self, callback: EventCallbackFn) {
		self.data.event_callback = Some(callback);
	}

	/// Sets VSync to be enabled or disabled
	pub fn set_vsync(&mut self, enabled: bool) {
		if enabled {
			self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
		} else {
			self.glfw.set_swap_interval(glfw::SwapInterval::None);
		}

		self.data.vsync = enabled;
	}

	/// Gets whether VSync is enabled or disabled
	pub fn get_vsync(&self) -> bool {
		self.data.vsync
	}
}
