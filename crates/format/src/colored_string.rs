use std::{
	fmt::{self, Display},
	ops::Deref,
};

use colored::Colorize;

/// Represents colored text for a specific color and mode.
pub struct ColoredString {
	text: String,
	mode: ColorMode,
	color: Color,
}

impl ColoredString {
	/// Creates a new blue text for a specific mode.
	pub fn blue(text: impl Into<String>, mode: ColorMode) -> Self {
		Self {
			text: text.into(),
			mode,
			color: Color::Blue,
		}
	}

	/// Creates a new red text for a specific mode.
	pub fn red(text: impl Into<String>, mode: ColorMode) -> Self {
		Self {
			text: text.into(),
			mode,
			color: Color::Red,
		}
	}
}

// `Deref` is necessary for padding in format strings to work correctly.
impl Deref for ColoredString {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.text
	}
}

impl Display for ColoredString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.mode {
			ColorMode::None => <String as fmt::Display>::fmt(&self.text, f),
			ColorMode::ANSI => match self.color {
				Color::Blue => <colored::ColoredString as fmt::Display>::fmt(&self.text.blue(), f),
				Color::Red => <colored::ColoredString as fmt::Display>::fmt(&self.text.red(), f),
			},
			ColorMode::HTML => {
				write!(f, "<span class=\"{}\">", self.color.get_css_class())?;
				<String as fmt::Display>::fmt(&self.text, f)?;
				write!(f, "</span>")
			}
		}
	}
}

/// Represents the currently supported color modes.  
/// `None` means that the text is not modified at all.  
/// For `ANSI`, the `colored` crate is used.  
/// For `HTML`, the text is wrapped as `<span class="{color}">{text}</span>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
	None,
	ANSI,
	HTML,
}

/// Represents the supported colors.
#[derive(Debug, Clone, Copy)]
pub enum Color {
	Blue,
	Red,
}

impl Color {
	/// Returns the corresponding CSS class name as string.
	/// Used for the HTML color mode.
	pub fn get_css_class(&self) -> &'static str {
		match self {
			Color::Blue => "blue",
			Color::Red => "red",
		}
	}
}
