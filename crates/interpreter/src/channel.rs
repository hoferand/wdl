use std::sync::Arc;

use tokio::sync::{
	mpsc::{self, Receiver, Sender},
	Mutex,
};

use crate::Value;

#[derive(Debug, Clone)]
pub struct Channel {
	tx: Sender<Value>,
	rx: Arc<Mutex<Receiver<Value>>>,
}

impl Channel {
	pub fn new(buffer: usize) -> Self {
		let (tx, rx) = mpsc::channel(buffer);

		Self {
			tx: tx,
			rx: Arc::new(Mutex::new(rx)),
		}
	}

	pub async fn send(&self, value: Value) -> Option<()> {
		self.tx.send(value).await.ok()
	}

	pub async fn receive(&self) -> Option<Value> {
		self.rx.lock().await.recv().await
	}
}
