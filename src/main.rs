mod server;
mod test;
mod screen;

use server::AppServer;

#[tokio::main]
async fn main() {
    let app = AppServer::new().await;
    setup_routes(app).serve().await;
}

fn setup_routes(app: AppServer) -> AppServer {
    app.merge(test::routes::router())
        .merge(screen::routes::router())
}
