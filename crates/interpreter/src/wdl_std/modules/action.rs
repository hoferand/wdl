use serde::Deserialize;

use ast::Identifier;
use logger::log;
use logger::Colorize;

use crate::{
	wdl_std::{call_function, get_handler, id, Arg, ArgType, Env},
	Error, FunctionId, FunctionValue, Value,
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Events {
	no_station_left: Option<FunctionId>,
}

impl<'de> ArgType<'de> for Events {}

async fn pickup(
	Env(env): Env,
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
) -> Result<(), Error> {
	log!("pickup from {:?}", target.val);
	log!("events {:?}", events.as_ref().map(|e| &e.val));

	if let Some(events) = events {
		if let Some(callback) = events.val.no_station_left {
			let ret = call_function(
				&callback,
				vec![Value::String(
					"Oh no, no station left for pickup!".to_owned(),
				)],
				Identifier("no_station_left".to_owned()),
				events.span,
				&env,
			)
			.await?;

			log!("Return value: {:?}", ret);
		}
	}

	Ok(())
}

async fn drop(target: Arg<Target, { id(b"target") }>) {
	log!("drop to {:?}", target.val);
}

async fn drive(target: Arg<Target, { id(b"target") }>) {
	log!("drive to {:?}", target.val);
}
