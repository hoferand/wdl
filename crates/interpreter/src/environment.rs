use std::{
	collections::HashMap,
	sync::{
		atomic::{AtomicU32, Ordering},
		Arc,
	},
};

use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Identifier, Node, ScopedIdentifier, Span};

use crate::{
	wdl_std::resolve_id, Channel, ChannelId, Error, ErrorKind, FunctionId, FunctionValue, Value,
};

// TODO: split into env and scope
pub struct Environment {
	parent: Option<Arc<Environment>>,
	variables: Arc<RwLock<HashMap<Identifier, Value>>>,
	functions: Arc<RwLock<HashMap<Identifier, FunctionValue>>>,
	channels: Arc<RwLock<HashMap<ChannelId, Channel>>>,
	channel_id: AtomicU32,
}

impl Default for Environment {
	fn default() -> Self {
		Self::new()
	}
}

impl Environment {
	pub fn new() -> Self {
		Environment {
			parent: None,
			variables: Arc::new(RwLock::new(HashMap::new())),
			functions: Arc::new(RwLock::new(HashMap::new())),
			channels: Arc::new(RwLock::new(HashMap::new())),
			channel_id: AtomicU32::new(0),
		}
	}

	pub fn with_parent(parent: Arc<Environment>) -> Self {
		Environment {
			parent: Some(parent),
			variables: Arc::new(RwLock::new(HashMap::new())),
			functions: Arc::new(RwLock::new(HashMap::new())),
			channels: Arc::new(RwLock::new(HashMap::new())),
			channel_id: AtomicU32::new(0),
		}
	}

	pub async fn declare(&self, id: Node<Span, Identifier>, val: Value) -> Result<(), Error> {
		// TODO: check if variable shadowing should be allowed
		let mut lock = self.variables.write().await;
		if lock.contains_key(&id.val) {
			return Err(Error {
				kind: ErrorKind::VariableAlreadyInUse { id: id.val },
				src: Some(id.src),
			});
		}
		lock.insert(id.val, val);

		Ok(())
	}

	pub async fn assign(&self, id: Node<Span, Identifier>, val: Value) -> Result<(), Error> {
		let Some(env) = self.resolve(&id.val).await else {
			return Err(Error {
				kind: ErrorKind::VariableNotFound {
					id: ScopedIdentifier {
						id: id.clone(),
						scope: Vec::new(),
					},
				},
				src: Some(id.src),
			});
		};
		env.write().await.insert(id.val, val);

		Ok(())
	}

	pub async fn get(&self, id: &Node<Span, Identifier>) -> Option<Value> {
		if let Some(env) = self.resolve(&id.val).await {
			if let Some(value) = env.read().await.get(&id.val) {
				return Some(value.clone());
			}
		}

		None
	}

	#[async_recursion]
	async fn resolve(&self, id: &Identifier) -> Option<Arc<RwLock<HashMap<Identifier, Value>>>> {
		if self.variables.read().await.contains_key(id) {
			Some(Arc::clone(&self.variables))
		} else if let Some(parent) = &self.parent {
			parent.resolve(id).await
		} else {
			None
		}
	}

	pub async fn declare_fn(
		&self,
		id: Node<Span, Identifier>,
		val: FunctionValue,
	) -> Result<(), Error> {
		self.declare(id.clone(), Value::Function(id.val.clone().into()))
			.await?;

		let mut lock = self.functions.write().await;
		if lock.contains_key(&id.val) {
			return Err(Error {
				kind: ErrorKind::VariableAlreadyInUse { id: id.val },
				src: Some(id.src),
			});
		}
		lock.insert(id.val, val);

		Ok(())
	}

	pub async fn get_fn(&self, id: &FunctionId) -> Option<FunctionValue> {
		if id.scope.len() == 0 {
			if let Some(value) = self.functions.read().await.get(&id.id) {
				return Some(value.clone());
			}
		}

		resolve_id(id)
	}

	pub async fn create_ch(&self, buffer: usize) -> (ChannelId, Channel) {
		let ch = Channel::new(buffer);
		let id = ChannelId(self.channel_id.fetch_add(1, Ordering::Relaxed));

		let mut lock = self.channels.write().await;
		lock.insert(id.clone(), ch.clone());

		(id, ch)
	}

	pub async fn get_ch(&self, id: &ChannelId) -> Option<Channel> {
		self.channels.read().await.get(id).cloned()
	}
}
