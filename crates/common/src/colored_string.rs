use std::{
	fmt::{self, Display},
	ops::Deref,
};

use colored::Colorize;

pub struct ColoredString {
	text: String,
	target: Target,
	color: Color,
}

impl ColoredString {
	pub fn blue(text: impl Into<String>, target: Target) -> Self {
		Self {
			text: text.into(),
			target,
			color: Color::Blue,
		}
	}

	pub fn red(text: impl Into<String>, target: Target) -> Self {
		Self {
			text: text.into(),
			target,
			color: Color::Red,
		}
	}
}

impl Deref for ColoredString {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.text
	}
}

impl Display for ColoredString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.target {
			Target::None => <String as fmt::Display>::fmt(&self.text, f),
			Target::ANSI => match self.color {
				Color::Blue => <colored::ColoredString as fmt::Display>::fmt(&self.text.blue(), f),
				Color::Red => <colored::ColoredString as fmt::Display>::fmt(&self.text.red(), f),
			},
			Target::HTML => {
				write!(f, "<span class=\"{}\">", self.color.get_css_class())?;
				<String as fmt::Display>::fmt(&self.text, f)?;
				write!(f, "</span>")
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Target {
	None,
	ANSI,
	HTML,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
	Blue,
	Red,
}

impl Color {
	pub fn get_css_class(&self) -> &'static str {
		match self {
			Color::Blue => "blue",
			Color::Red => "red",
		}
	}
}
