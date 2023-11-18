pub use colored::Colorize;

#[macro_export]
macro_rules! trace {
	($($arg:tt)*) => {{
		println!("{}: {}", "TRACE".green().bold(), format_args!($($arg)*));
	}};
}

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {{
		println!("{}: {}", "LOG".blue().bold(), format_args!($($arg)*));
	}};
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {{
		eprintln!("{}: {}", "ERROR".red().bold(), format_args!($($arg)*));
	}};
}

#[macro_export]
macro_rules! warning {
	($($arg:tt)*) => {{
		eprintln!("{}: {}", "WARNING".truecolor(255, 140, 0).bold(), format_args!($($arg)*));
	}};
}
