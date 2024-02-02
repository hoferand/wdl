use std::time::Duration;

pub async fn sleep(millis: f64) {
	tokio::time::sleep(Duration::from_millis(millis as u64)).await; // TODO: fix millis as u64
}
