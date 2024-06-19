mod server;
mod test;

use server::AppServer;

#[tokio::main]
async fn main() {
    let app = AppServer::new().await;
    setup_routes(app).serve().await;
}

fn setup_routes(app: AppServer) -> AppServer {
    app.merge(test::routes::router())
}
