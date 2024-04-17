use ast::Span;
use serde::Deserialize;

use logger::log;
use logger::Colorize;

use ast::ScopedIdentifier;

use crate::{wdl_std::get_handler, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier<Span>) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"pickup" => Some(get_handler(pickup)),
		"drop" => Some(get_handler(drop)),
		"drive" => Some(get_handler(drive)),
		_ => None,
	}
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Target {
	stations: Option<Vec<String>>,
	stationareas: Option<Vec<String>>,
	coordinates: Option<Vec<Coordinate>>,
	not: Option<Box<TargetNot>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetNot {
	stations: Option<Vec<String>>,
	stationareas: Option<Vec<String>>,
	coordinates: Option<Vec<Coordinate>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Coordinate {
	x: u32,
	y: u32,
}

async fn pickup(target: Target) -> Result<(), Error> {
	log!("pickup from {:?}", target);
	Ok(())
}

async fn drop(target: Target) -> Result<(), Error> {
	log!("drop to {:?}", target);
	Ok(())
}

async fn drive(target: Target) -> Result<(), Error> {
	log!("drive to {:?}", target);
	Ok(())
}
