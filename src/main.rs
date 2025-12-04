mod routes;

use std::env;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build routes
    let app = Router::new()
        .merge(routes::health::routes())
        .merge(routes::fridge::routes());

    // address & port
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on http://{}", addr);

    // run server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
