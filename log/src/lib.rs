macro_rules! meta_log_macro {
	($d:tt $i:ident, $msg:tt) => {
		#[macro_export]
		macro_rules! $i {
			( $d ( $arg:tt )+ ) => {
				println!($msg, format!($d ( $arg )+) )
			};
		}
	};
}

meta_log_macro!($ core_trace, "[LANTERN | TRACE] {}");
meta_log_macro!($ core_info, "[LANTERN | INFO] {}");
meta_log_macro!($ core_warn, "[LANTERN | WARN] {}");
meta_log_macro!($ core_error, "[LANTERN | ERROR] {}");
meta_log_macro!($ core_fatal, "[LANTERN | FATAL] {}");
meta_log_macro!($ trace, "[APP | TRACE] {}");
meta_log_macro!($ info, "[APP | INFO] {}");
meta_log_macro!($ warn, "[APP | WARN] {}");
meta_log_macro!($ error, "[APP | ERROR] {}");
meta_log_macro!($ fatal, "[APP | FATAL] {}");
