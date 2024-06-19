use std::net::{Ipv4Addr, SocketAddrV4};

use axum::Router;

const LOCALHOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

pub struct AppServer {
    app: Router,
    listener: tokio::net::TcpListener,
    routers: Vec<Router>,
}

impl AppServer {
    pub async fn new() -> Self {
        AppServer {
            app: Router::new(),
            listener: tokio::net::TcpListener::bind(LOCALHOST).await.unwrap(),
            routers: Vec::new(),
        }
    }

    pub async fn serve(self) {
        axum::serve::serve(self.listener, self.app).await;
    }

    pub fn merge(mut self, router: Router) -> Self {
        self.app = self.app.merge(router);
        self
    }
}
