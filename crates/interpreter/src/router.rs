use router::{RouterClient, RouterClientGrpc, RouterClientWs};

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
