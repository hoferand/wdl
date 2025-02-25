use std::sync::Arc;

use tokio::sync::{
	Mutex,
	mpsc::{self, Receiver, Sender},
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
			tx,
			rx: Arc::new(Mutex::new(rx)),
		}
	}

	pub async fn send(&self, value: Value) -> Option<()> {
		self.tx.send(value).await.ok()
	}

	pub async fn receive(&self) -> Option<Value> {
		self.rx.lock().await.recv().await
	}

	pub async fn close(&self) {
		self.rx.lock().await.close();
	}
}
