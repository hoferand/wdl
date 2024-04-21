use serde::Deserialize;

use ast::Identifier;
use logger::log;
use logger::Colorize;
use router::{self, PickupStatus, Target};

use crate::{
	wdl_std::{call_function, get_handler, id, Arg, ArgType, Env, Source},
	Error, ErrorKind, FunctionId, FunctionValue, Value,
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
	log!("pickup from {:?}", target.val);

	let status = match router::pickup(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				src: Some(src),
			});
		}
	};

	log!("pickup status: `{:?}`", status);

	if let Some(events) = events {
		if status == PickupStatus::NoStationLeft {
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
	}

	Ok(())
}

async fn drop(target: Arg<Target, { id(b"target") }>) {
	log!("drop to {:?}", target.val);
}

async fn drive(target: Arg<Target, { id(b"target") }>) {
	log!("drive to {:?}", target.val);
}
