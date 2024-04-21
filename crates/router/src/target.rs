use serde::{Deserialize, Serialize};

use crate::proto;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
	pub stations: Option<Vec<String>>,
	pub stationareas: Option<Vec<String>>,
	pub coordinates: Option<Vec<Coordinate>>,
	pub not: Option<NotTarget>,
}

impl From<Target> for proto::Target {
	fn from(target: Target) -> Self {
		proto::Target {
			stations: target.stations.unwrap_or_default(),
			stationareas: target.stationareas.unwrap_or_default(),
			coordinates: target
				.coordinates
				.unwrap_or_default()
				.into_iter()
				.map(Coordinate::into)
				.collect(),
			not: target.not.map(NotTarget::into),
		}
	}
}

impl From<proto::Target> for Target {
	fn from(target: proto::Target) -> Self {
		let mut stations = None;
		if !target.stations.is_empty() {
			stations = Some(target.stations);
		}
		let mut stationareas = None;
		if !target.stationareas.is_empty() {
			stationareas = Some(target.stationareas);
		}
		let mut coordinates = None;
		if !target.coordinates.is_empty() {
			let mut coords = Vec::new();
			for coord in target.coordinates {
				coords.push(coord.into());
			}

			coordinates = Some(coords);
		}
		Target {
			stations,
			stationareas,
			coordinates,
			not: target.not.map(proto::NotTarget::into),
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NotTarget {
	pub stations: Option<Vec<String>>,
	pub stationareas: Option<Vec<String>>,
	pub coordinates: Option<Vec<Coordinate>>,
}

impl From<NotTarget> for proto::NotTarget {
	fn from(target: NotTarget) -> Self {
		proto::NotTarget {
			stations: target.stations.unwrap_or_default(),
			stationareas: target.stationareas.unwrap_or_default(),
			coordinates: target
				.coordinates
				.unwrap_or_default()
				.into_iter()
				.map(Coordinate::into)
				.collect(),
		}
	}
}

impl From<proto::NotTarget> for NotTarget {
	fn from(target: proto::NotTarget) -> Self {
		let mut stations = None;
		if !target.stations.is_empty() {
			stations = Some(target.stations);
		}
		let mut stationareas = None;
		if !target.stationareas.is_empty() {
			stationareas = Some(target.stationareas);
		}
		let mut coordinates = None;
		if !target.coordinates.is_empty() {
			let mut coords = Vec::new();
			for coord in target.coordinates {
				coords.push(coord.into());
			}

			coordinates = Some(coords);
		}
		NotTarget {
			stations,
			stationareas,
			coordinates,
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coordinate {
	pub x: f64, // TODO: change to u32
	pub y: f64, // TODO: change to u32
}

impl From<Coordinate> for proto::Coordinate {
	fn from(coord: Coordinate) -> Self {
		proto::Coordinate {
			x: coord.x,
			y: coord.y,
		}
	}
}

impl From<proto::Coordinate> for Coordinate {
	fn from(coord: proto::Coordinate) -> Self {
		Coordinate {
			x: coord.x,
			y: coord.y,
		}
	}
}
