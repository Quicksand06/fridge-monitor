mod routes;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build routes
    let app = Router::new()
        .merge(routes::health::routes())
        .merge(routes::fridge::routes());

    // address & port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    // run server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
