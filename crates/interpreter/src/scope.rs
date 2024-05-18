use std::{collections::HashMap, sync::Arc};

use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Identifier, Node, Variable};

use crate::{Error, ErrorKind, Value};

pub struct Scope {
	parent: Option<Arc<Scope>>,
	variables: RwLock<HashMap<Identifier, Value>>,
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

impl Scope {
	pub fn new() -> Self {
		Self {
			parent: None,
			variables: RwLock::new(HashMap::new()),
		}
	}

	pub fn with_parent(parent: Arc<Scope>) -> Self {
		Self {
			parent: Some(parent),
			variables: RwLock::new(HashMap::new()),
		}
	}

	pub async fn declare(&self, id: Node<Identifier>, val: Value) -> Result<(), Error> {
		// TODO: check if variable shadowing should be allowed
		let mut lock = self.variables.write().await;
		if lock.contains_key(&id.val) {
			return Err(Error {
				kind: ErrorKind::VariableAlreadyInUse { id: id.val },
				src: Some(id.span),
			});
		}
		lock.insert(id.val, val);

		Ok(())
	}

	pub async fn assign(&self, id: Node<Identifier>, val: Value) -> Result<(), Error> {
		let Some(env) = self.resolve(&id.val).await else {
			return Err(Error {
				kind: ErrorKind::VariableNotFound {
					id: Variable {
						id: id.clone(),
						scope: Vec::new(),
					},
				},
				src: Some(id.span),
			});
		};
		env.write().await.insert(id.val, val);

		Ok(())
	}

	pub async fn get(&self, id: &Node<Identifier>) -> Option<Value> {
		if let Some(env) = self.resolve(&id.val).await {
			if let Some(value) = env.read().await.get(&id.val) {
				return Some(value.clone());
			}
		}

		None
	}

	#[async_recursion]
	async fn resolve(&self, id: &Identifier) -> Option<&RwLock<HashMap<Identifier, Value>>> {
		if self.variables.read().await.contains_key(id) {
			Some(&self.variables)
		} else if let Some(parent) = &self.parent {
			parent.resolve(id).await
		} else {
			None
		}
	}
}
