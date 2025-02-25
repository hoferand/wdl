use std::{collections::HashMap, sync::Arc};

use log::info;
use serde::Deserialize;

use ast::{Identifier, Span};
use router::{self, RouterClient, RouterStatus, Target};

use crate::{
	Environment, Error, ErrorKind, FunctionId, FunctionValue, LogEntry, Value,
	wdl_std::{Arg, ArgType, call_function, get_handler, id},
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

impl ArgType<'_> for Events {}

async fn pickup(
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
	fn_span: Span,
	env: Arc<Environment>,
) -> Result<(), Error> {
	info!("pickup from {:?}", target.val);

	env.send_log(LogEntry::info("Start pickup action.", Some(fn_span)))
		.await;

	let status = match env.router.pickup(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				span: Some(fn_span),
			});
		}
	};

	info!("pickup status: `{:?}`", status);

	let mut hooked = false;
	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			hooked = true;

			if let Some(callback) = events.val.no_station_left {
				env.send_log(LogEntry::info(
					format!("Trigger `no_station_left`, execute `{}`.", callback),
					Some(fn_span),
				))
				.await;

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

	if !hooked && status == RouterStatus::NoStationLeft {
		env.send_log(LogEntry::info(
			"Trigger `no_station_left`, not hooked.",
			Some(fn_span),
		))
		.await;
	}

	env.send_log(LogEntry::info("Pickup action finished.", Some(fn_span)))
		.await;

	Ok(())
}

async fn drop(
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
	fn_span: Span,
	env: Arc<Environment>,
) -> Result<(), Error> {
	info!("drop to {:?}", target.val);

	env.send_log(LogEntry::info("Start drop action.", Some(fn_span)))
		.await;

	let status = match env.router.drop(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				span: Some(fn_span),
			});
		}
	};

	info!("drop status: `{:?}`", status);

	let mut hooked = false;
	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			hooked = true;

			if let Some(callback) = events.val.no_station_left {
				env.send_log(LogEntry::info(
					format!("Trigger `no_station_left`, execute `{}`.", callback),
					Some(fn_span),
				))
				.await;

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

	if !hooked && status == RouterStatus::NoStationLeft {
		env.send_log(LogEntry::info(
			"Trigger `no_station_left`, not hooked.",
			Some(fn_span),
		))
		.await;
	}

	env.send_log(LogEntry::info("Drop action finished.", Some(fn_span)))
		.await;

	Ok(())
}

async fn drive(
	target: Arg<Target, { id(b"target") }>,
	events: Option<Arg<Events, { id(b"events") }>>,
	fn_span: Span,
	env: Arc<Environment>,
) -> Result<(), Error> {
	info!("drive to {:?}", target.val);

	env.send_log(LogEntry::info("Start drive action.", Some(fn_span)))
		.await;

	let status = match env.router.drive(target.val).await {
		Some(s) => s,
		None => {
			return Err(Error {
				kind: ErrorKind::Fatal("Communication with router failed".to_owned()),
				span: Some(fn_span),
			});
		}
	};

	info!("drive status: `{:?}`", status);

	let mut hooked = false;
	if let Some(events) = events {
		if status == RouterStatus::NoStationLeft {
			hooked = true;

			if let Some(callback) = events.val.no_station_left {
				env.send_log(LogEntry::info(
					format!("Trigger `no_station_left`, execute `{}`.", callback),
					Some(fn_span),
				))
				.await;

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

	if !hooked && status == RouterStatus::NoStationLeft {
		env.send_log(LogEntry::info(
			"Trigger `no_station_left`, not hooked.",
			Some(fn_span),
		))
		.await;
	}

	env.send_log(LogEntry::info("Drive action finished.", Some(fn_span)))
		.await;

	Ok(())
}
