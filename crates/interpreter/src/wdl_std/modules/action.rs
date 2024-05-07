use std::collections::HashMap;

use log::info;
use serde::Deserialize;

use ast::Identifier;
use router::{self, RouterClient, RouterStatus, Target};

use crate::{
	wdl_std::{call_function, get_handler, id, Arg, ArgType, Env, Source},
	Error, ErrorKind, FunctionId, FunctionValue, Value,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"pickup" => Some(get_handler(pickup)),
		"drop" => Some(get_handler(drop)),
		"drive" => Some(get_handler(drive)),
		_ => None,
	}
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Events {
	pub no_station_left: Option<FunctionId>,
}

impl<'de> ArgType<'de> for Events {}

async fn pickup(
	Source(src): Source,
	Env(env): Env,
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
) -> Result<(), Error> {
	info!("pickup from {:?}", target.val);

	let status = match env.router.pickup(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				src: Some(src),
			});
		}
	};

	info!("pickup status: `{:?}`", status);

	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			if let Some(callback) = events.val.no_station_left {
				let mut event = HashMap::new();
				event.insert(
					"type".to_owned(),
					Value::String("NoStationLeftEvent".to_owned()),
				);
				let ret = call_function(
					&callback,
					vec![Value::Object(event)],
					Identifier {
						id: "no_station_left".to_owned(),
					},
					events.span,
					&env,
				)
				.await?;

				info!("Return value of no_station_left handler: {:?}", ret);
			}
		}
	}

	Ok(())
}

async fn drop(
	Source(src): Source,
	Env(env): Env,
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
) -> Result<(), Error> {
	info!("drop to {:?}", target.val);

	let status = match env.router.drop(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				src: Some(src),
			});
		}
	};

	info!("drop status: `{:?}`", status);

	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			if let Some(callback) = events.val.no_station_left {
				let mut event = HashMap::new();
				event.insert(
					"type".to_owned(),
					Value::String("NoStationLeftEvent".to_owned()),
				);
				let ret = call_function(
					&callback,
					vec![Value::Object(event)],
					Identifier {
						id: "no_station_left".to_owned(),
					},
					events.span,
					&env,
				)
				.await?;

				info!("Return value of no_station_left handler: {:?}", ret);
			}
		}
	}

	Ok(())
}

async fn drive(
	Source(src): Source,
	Env(env): Env,
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
) -> Result<(), Error> {
	info!("drive to {:?}", target.val);

	let status = match env.router.drive(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				src: Some(src),
			});
		}
	};

	info!("drive status: `{:?}`", status);

	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			if let Some(callback) = events.val.no_station_left {
				let mut event = HashMap::new();
				event.insert(
					"type".to_owned(),
					Value::String("NoStationLeftEvent".to_owned()),
				);
				let ret = call_function(
					&callback,
					vec![Value::Object(event)],
					Identifier {
						id: "no_station_left".to_owned(),
					},
					events.span,
					&env,
				)
				.await?;

				info!("Return value of no_station_left handler: {:?}", ret);
			}
		}
	}

	Ok(())
}
