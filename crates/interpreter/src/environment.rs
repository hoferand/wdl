use std::{collections::HashMap, sync::Arc};

use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Identifier, Node};

use crate::{Error, Value};

pub struct Environment<'p> {
	parent: Option<&'p Environment<'p>>,
	variables: Arc<RwLock<HashMap<Identifier, Value>>>,
}

impl<'p> Environment<'p> {
	pub fn new() -> Self {
		Environment {
			parent: None,
			variables: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub fn with_parent(parent: &'p Environment) -> Self {
		Environment {
			parent: Some(parent),
			variables: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	pub async fn declare(&self, id: Node<Identifier>, val: Value) -> Result<(), Error> {
		// TODO: check if variable shadowing should be allowed
		let mut lock = self.variables.write().await;
		if lock.contains_key(&id.val) {
			return Err(Error::VariableAlreadyInUse {
				id: id.val,
				span: id.span,
			});
		}
		lock.insert(id.val, val);

		Ok(())
	}

	pub async fn assign(&self, id: Node<Identifier>, val: Value) -> Result<(), Error> {
		let Some(env) = self.resolve(&id.val).await else {
			return Err(Error::VariableNotFound {
				id: id.val,
				span: id.span,
			});
		};
		env.write().await.insert(id.val, val);

		Ok(())
	}

	pub async fn get(&self, id: &Node<Identifier>) -> Result<Value, Error> {
		if let Some(env) = self.resolve(&id.val).await {
			if let Some(value) = env.read().await.get(&id.val) {
				return Ok(value.clone());
			}
		}

		Err(Error::VariableNotFound {
			id: id.val.clone(),
			span: id.span.clone(),
		})
	}

	#[async_recursion]
	async fn resolve(&self, id: &Identifier) -> Option<&Arc<RwLock<HashMap<Identifier, Value>>>> {
		if self.variables.read().await.contains_key(id) {
			Some(&self.variables)
		} else if let Some(parent) = self.parent {
			parent.resolve(id).await
		} else {
			None
		}
	}
}
