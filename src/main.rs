mod models;
mod routes;

use axum::Router;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;

    // Postgres pool
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // build routes
    let app = Router::<PgPool>::new()
        .merge(routes::health::routes())
        .merge(routes::fridge::routes())
        .with_state(db);

    // address & port
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let ip = env::var("HOST_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr: SocketAddr = format!("{}:{}", ip, port)
        .parse()
        .expect("Invalid IP or port");
    println!("listening on http://{}", addr);

    // run server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
