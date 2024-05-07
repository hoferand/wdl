use std::{
	collections::HashMap,
	sync::{
		atomic::{AtomicU32, Ordering},
		Arc,
	},
};

use log::error;
use tokio::{
	sync::{mpsc::Sender, Mutex, RwLock},
	task::JoinHandle,
};

use ast::{Identifier, Node, Span};
use router::{RouterClient, RouterClientGrpc, RouterClientWs};

use crate::{
	wdl_std::resolve_id, Channel, ChannelId, Error, ErrorKind, FunctionId, FunctionValue, Scope,
	Value,
};

pub struct Environment {
	pub global_scope: Arc<Scope>,
	pub router: Router,
	error_ch: Mutex<Option<Sender<Error>>>, // TODO: remove Option
	handles: Mutex<Vec<JoinHandle<Result<(), Error>>>>,
	functions: RwLock<HashMap<Identifier, FunctionValue>>,
	channels: RwLock<HashMap<ChannelId, Channel>>,
	channel_id: AtomicU32,
}

impl Environment {
	pub fn new(global_scope: Arc<Scope>, router: Router) -> Self {
		Environment {
			global_scope,
			router,
			error_ch: Mutex::new(None),
			handles: Mutex::new(Vec::new()),
			functions: RwLock::new(HashMap::new()),
			channels: RwLock::new(HashMap::new()),
			channel_id: AtomicU32::new(0),
		}
	}

	pub async fn push_handle(&self, handle: JoinHandle<Result<(), Error>>) {
		self.handles.lock().await.push(handle);
	}

	pub async fn pop_handle(&self) -> Option<JoinHandle<Result<(), Error>>> {
		self.handles.lock().await.pop()
	}

	pub async fn set_error_ch(&self, ch: Sender<Error>) {
		*self.error_ch.lock().await = Some(ch);
	}

	pub async fn send_error(&self, err: Error) {
		if let Some(ch) = self.error_ch.lock().await.as_ref() {
			if let Err(send_err) = ch.send(err.clone()).await {
				error!(
					"Failed to send error over channel `{}` `{:?}`",
					send_err, err
				);
				// TODO: panic?
			}
		} else {
			error!("Error channel missing, cannot send error `{:?}`", err);
			// TODO: panic?
		}
	}

	pub async fn declare_fn(
		&self,
		id: Node<Span, Identifier>,
		val: FunctionValue,
	) -> Result<(), Error> {
		self.global_scope
			.declare(id.clone(), Value::Function(id.val.clone().into()))
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
		if id.scope.is_empty() {
			if let Some(value) = self.functions.read().await.get(&id.id) {
				return Some(value.clone());
			}
		}

		resolve_id(id)
	}

	pub async fn create_ch(&self, buffer: usize) -> (ChannelId, Channel) {
		let ch = Channel::new(buffer);
		let id = ChannelId {
			id: self.channel_id.fetch_add(1, Ordering::Relaxed),
		};

		let mut lock = self.channels.write().await;
		lock.insert(id.clone(), ch.clone());

		(id, ch)
	}

	pub async fn get_ch(&self, id: &ChannelId) -> Option<Channel> {
		self.channels.read().await.get(id).cloned()
	}
}

pub enum Router {
	Grpc(RouterClientGrpc),
	Ws(RouterClientWs),
}

impl RouterClient for Router {
	async fn pickup(&self, target: router::Target) -> Option<router::RouterStatus> {
		match self {
			Router::Grpc(router) => router.pickup(target).await,
			Router::Ws(router) => router.pickup(target).await,
		}
	}

	async fn drop(&self, target: router::Target) -> Option<router::RouterStatus> {
		match self {
			Router::Grpc(router) => router.drop(target).await,
			Router::Ws(router) => router.drop(target).await,
		}
	}

	async fn drive(&self, target: router::Target) -> Option<router::RouterStatus> {
		match self {
			Router::Grpc(router) => router.drive(target).await,
			Router::Ws(router) => router.drive(target).await,
		}
	}
}
