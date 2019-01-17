use chrono::{ Utc, DateTime, Timelike };

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

macro_rules! meta_log_macro {
	($d:tt $i:ident, $msg:tt) => {
		#[macro_export]
		macro_rules! $i {
			( $d ( $arg:tt )+ ) => {
				let now = $crate::get_current_time();
				print!("[{:02}:{:02}:{:02}] - ", $crate::get_hour(&now), $crate::get_min(&now), $crate::get_sec(&now));
				println!($msg, $d( $arg )+ );
			};
		}
	};
}

meta_log_macro!($ core_trace, "LANTERN | TRACE: {}");
meta_log_macro!($ core_info, "LANTERN | INFO: {}");
meta_log_macro!($ core_warn, "LANTERN | WARN: {}");
meta_log_macro!($ core_error, "LANTERN | ERROR: {}");
meta_log_macro!($ core_fatal, "LANTERN | FATAL: {}");
meta_log_macro!($ trace, "APP | TRACE: {}");
meta_log_macro!($ info, "APP | INFO: {}");
meta_log_macro!($ warn, "APP | WARN: {}");
meta_log_macro!($ error, "APP | ERROR: {}");
meta_log_macro!($ fatal, "APP | FATAL: {}");
