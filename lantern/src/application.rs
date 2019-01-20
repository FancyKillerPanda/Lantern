use crate::events;

pub trait Application {
	fn run(&self) {
		let win_resize_event = events::WindowResizeEvent::new(events::Size(1280, 720));
		let win_closed_event = events::WindowClosedEvent::new();
		let app_tick_event = events::AppTickEvent::new();
		let app_update_event = events::AppUpdateEvent::new();
		let app_render_event = events::AppRenderEvent::new();

		println!("\n");		
		log::trace!("{}", win_resize_event.to_string());
		log::trace!("{}", win_closed_event.to_string());
		log::trace!("{}", app_tick_event.to_string());
		log::trace!("{}", app_update_event.to_string());
		log::trace!("{}", app_render_event.to_string());

		let key_pressed_event = events::KeyPressedEvent::new(1, 0);
		let key_released_event = events::KeyReleasedEvent::new(1);

		println!("");
		log::trace!("{}", key_pressed_event.to_string());
		log::trace!("{}", key_released_event.to_string());
	}
}
