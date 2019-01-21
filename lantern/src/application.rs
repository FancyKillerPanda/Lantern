use crate::events;

/// Main engine entry point
/// 
/// Application should be implemented by the client's main struct.
pub trait Application {
	/// Runs the engine
	fn run(&self) {
		// TEMPORARY CODE //
		
		// Creates one of each type of Application event
		let win_resize_event = events::WindowResizeEvent::new(events::Size(1280, 720));
		let win_closed_event = events::WindowClosedEvent::new();
		let app_tick_event = events::AppTickEvent::new();
		let app_update_event = events::AppUpdateEvent::new();
		let app_render_event = events::AppRenderEvent::new();

		// Logs the Application events
		println!("\n");		
		log::trace!("{}", win_resize_event.to_string());
		log::trace!("{}", win_closed_event.to_string());
		log::trace!("{}", app_tick_event.to_string());
		log::trace!("{}", app_update_event.to_string());
		log::trace!("{}", app_render_event.to_string());

		// Creates one of each type of Key event
		let key_pressed_event = events::KeyPressedEvent::new(1, 0);
		let key_released_event = events::KeyReleasedEvent::new(1);

		// Logs the Key events
		println!("");
		log::trace!("{}", key_pressed_event.to_string());
		log::trace!("{}", key_released_event.to_string());
	
		// Creates one of each type of Mouse event
		let mouse_moved_event = events::MouseMovedEvent::new(events::Position(400, 300));
		let mouse_scrolled_event = events::MouseScrolledEvent::new(events::Offset(0, 1));
		let mouse_btn_pressed_event = events::MouseButtonPressedEvent::new(1);
		let mouse_btn_released_event = events::MouseButtonReleasedEvent::new(1);

		// Logs the Mouse events
		println!("");
		log::trace!("{}", mouse_moved_event.to_string());
		log::trace!("{}", mouse_scrolled_event.to_string());
		log::trace!("{}", mouse_btn_pressed_event.to_string());
		log::trace!("{}", mouse_btn_released_event.to_string());
	}
}
