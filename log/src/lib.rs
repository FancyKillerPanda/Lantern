use chrono::{ Utc, DateTime, Timelike };
use ansi_term::{ Colour, Style };

pub fn get_current_time() -> DateTime<Utc> {
	Utc::now()
}

pub fn get_hour(time: &DateTime<Utc>) -> u32 {
	time.hour()
}

pub fn get_min(time: &DateTime<Utc>) -> u32 {
	time.minute()
}

pub fn get_sec(time: &DateTime<Utc>) -> u32 {
	time.second()
}

pub fn get_trace_colour() -> Style {
	Style::new().fg(Colour::RGB(192, 192, 192))
}

pub fn get_info_colour() -> Style {
	Style::new().fg(Colour::RGB(0, 128, 0))
}

pub fn get_warn_colour() -> Style {
	Style::new().fg(Colour::RGB(128, 128, 0)).bold()
}

pub fn get_error_colour() -> Style {
	Style::new().fg(Colour::RGB(128, 0, 0)).bold()
}

pub fn get_fatal_colour() -> Style {
	Style::new().on(Colour::RGB(128, 0, 0)).bold()
}

macro_rules! meta_log_macro {
	($d:tt $i:ident, $msg:tt, $col_fn:ident) => {
		#[macro_export]
		macro_rules! $i {
			( $d ( $arg:tt )+ ) => {
				let now = $crate::get_current_time();
				let log_information = $crate::$col_fn().paint(format!("[{:02}:{:02}:{:02}] - {}",
					$crate::get_hour(&now),
					$crate::get_min(&now),
					$crate::get_sec(&now),
					$msg
				));
				
				println!("{}: {}", log_information, format!($d( $arg )+));
			};
		}
	};
}

meta_log_macro!($ core_trace, "LANTERN | TRACE", get_trace_colour);
meta_log_macro!($ core_info, "LANTERN | INFO", get_info_colour);
meta_log_macro!($ core_warn, "LANTERN | WARN", get_warn_colour);
meta_log_macro!($ core_error, "LANTERN | ERROR", get_error_colour);
meta_log_macro!($ core_fatal, "LANTERN | FATAL", get_fatal_colour);
meta_log_macro!($ trace, "APP | TRACE", get_trace_colour);
meta_log_macro!($ info, "APP | INFO", get_info_colour);
meta_log_macro!($ warn, "APP | WARN", get_warn_colour);
meta_log_macro!($ error, "APP | ERROR", get_error_colour);
meta_log_macro!($ fatal, "APP | FATAL", get_fatal_colour);
