use std::{fmt::Debug, sync::Arc};

use ast::Function;

use crate::wdl_std::StdFunction;

#[derive(Clone)]
pub enum FunctionValue {
	Custom(Function),
	Std(Arc<dyn StdFunction + Send + Sync>),
}

impl PartialEq for FunctionValue {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

impl Debug for FunctionValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FunctionValue::Custom(_) => write!(f, "CustomFunction"),
			FunctionValue::Std(_) => write!(f, "StdFunction"),
		}
	}
}
