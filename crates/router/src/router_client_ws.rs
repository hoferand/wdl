use std::sync::Arc;

use serde::Serialize;
use tokio::sync::{
	mpsc::{self, Receiver, Sender},
	Mutex,
};

use crate::{RouterStatus, Target};

#[derive(Debug, Serialize)]
pub struct WsRouterRequest {
	action: String,
	target: Target,
}

pub struct RouterClientWs {
	receiver: Arc<Mutex<Receiver<RouterStatus>>>,
	sender: Sender<WsRouterRequest>,
}

impl RouterClientWs {
	pub fn new() -> (Sender<RouterStatus>, Receiver<WsRouterRequest>, Self) {
		let (tx1, receiver) = mpsc::channel(3);
		let (sender, rx2) = mpsc::channel(3);

		(
			tx1,
			rx2,
			Self {
				receiver: Arc::new(Mutex::new(receiver)),
				sender,
			},
		)
	}
}

impl crate::RouterClient for RouterClientWs {
	async fn pickup(&self, target: Target) -> Option<RouterStatus> {
		self.sender
			.send(WsRouterRequest {
				action: "Pickup".to_owned(),
				target,
			})
			.await
			.unwrap();

		self.receiver.lock().await.recv().await
	}

	async fn drop(&self, target: Target) -> Option<RouterStatus> {
		self.sender
			.send(WsRouterRequest {
				action: "Drop".to_owned(),
				target,
			})
			.await
			.ok();

		self.receiver.lock().await.recv().await
	}

	async fn drive(&self, target: Target) -> Option<RouterStatus> {
		self.sender
			.send(WsRouterRequest {
				action: "Drive".to_owned(),
				target,
			})
			.await
			.ok();

		self.receiver.lock().await.recv().await
	}
}
