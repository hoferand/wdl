use serde::Deserialize;

use logger::log;
use logger::Colorize;

use crate::{
	wdl_std::{get_handler, id, Arg, ArgType},
	FunctionId, FunctionValue,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.0.as_str() {
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

impl<'de> ArgType<'de> for Target {}

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

async fn pickup(target: Arg<Target, { id(b"target") }>) {
	log!("pickup from {:?}", target.val);
}

async fn drop(target: Arg<Target, { id(b"target") }>) {
	log!("drop to {:?}", target.val);
}

async fn drive(target: Arg<Target, { id(b"target") }>) {
	log!("drive to {:?}", target.val);
}
