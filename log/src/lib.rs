//! Logging Library
//! 
//! This is a small logging library made for the Lantern engine.
//! It features the ability to log at different severities, with
//! specialised info and colour for each one.

use chrono::{ Utc, DateTime, Timelike };
use ansi_term::{ Colour, Style };

/// Gets the current time
/// 
/// This function is a wrapper for `chrono::Utc::now()`
pub fn get_current_time() -> DateTime<Utc> {
	Utc::now()
}

/// Gets the hour from a `chrono::DateTime` reference
/// 
/// This function is a wrapper for chrono::DateTime::hour()`
pub fn get_hour(time: &DateTime<Utc>) -> u32 {
	time.hour()
}

/// Gets the minute from a `chrono::DateTime` reference
/// 
/// This function is a wrapper for chrono::DateTime::minute()`
pub fn get_min(time: &DateTime<Utc>) -> u32 {
	time.minute()
}

/// Gets the second from a `chrono::DateTime` reference
/// 
/// This function is a wrapper for chrono::DateTime::second()`
pub fn get_sec(time: &DateTime<Utc>) -> u32 {
	time.second()
}

/// Gets the terminal colour for the `trace!()` macro
pub fn get_trace_colour() -> Style {
	Style::new().fg(Colour::RGB(192, 192, 192))
}

/// Gets the terminal colour for the `info!()` macro
pub fn get_info_colour() -> Style {
	Style::new().fg(Colour::RGB(0, 128, 0))
}

/// Gets the terminal colour for the `warn!()` macro
pub fn get_warn_colour() -> Style {
	Style::new().fg(Colour::RGB(128, 128, 0)).bold()
}

/// Gets the terminal colour for the `error!()` macro
pub fn get_error_colour() -> Style {
	Style::new().fg(Colour::RGB(128, 0, 0)).bold()
}

/// Gets the terminal colour for the `fatal!()` macro
pub fn get_fatal_colour() -> Style {
	Style::new().on(Colour::RGB(128, 0, 0)).bold()
}

/// Creates a logging macro
macro_rules! meta_log_macro {
	($d:tt $i:ident, $msg:tt, $col_fn:ident) => {
		#[macro_export]
		macro_rules! $i {
			( $d ( $arg:tt )+ ) => {
				// Gets the current time
				let now = $crate::get_current_time();

				// Colours the information
				let log_information = $crate::$col_fn().paint(format!("[{:02}:{:02}:{:02}] - {}",
					$crate::get_hour(&now),
					$crate::get_min(&now),
					$crate::get_sec(&now),
					$msg
				));
				
				// Prints the information and the message given
				println!("{}: {}", log_information, format!($d( $arg )+));
			};
		}
	};
}

// Trace level macro used by Lantern
meta_log_macro!($ core_trace, "LANTERN | TRACE", get_trace_colour);
// Information level macro used by Lantern
meta_log_macro!($ core_info, "LANTERN | INFO", get_info_colour);
// Warning level macro used by Lantern
meta_log_macro!($ core_warn, "LANTERN | WARN", get_warn_colour);
// Error level macro used by Lantern
meta_log_macro!($ core_error, "LANTERN | ERROR", get_error_colour);
// Critical level macro used by Lantern
meta_log_macro!($ core_fatal, "LANTERN | FATAL", get_fatal_colour);
// Trace level macro used by the application
meta_log_macro!($ trace, "APP | TRACE", get_trace_colour);
// Information level macro used by the application
meta_log_macro!($ info, "APP | INFO", get_info_colour);
// Warning level macro used by the application
meta_log_macro!($ warn, "APP | WARN", get_warn_colour);
// Error level macro used by the application
meta_log_macro!($ error, "APP | ERROR", get_error_colour);
// Critical level macro used by the application
meta_log_macro!($ fatal, "APP | FATAL", get_fatal_colour);
