mod db;
mod handlers;
mod models;


use axum::{Router, routing::{get, post}};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, ConnectOptions};
use std::{env, net::SocketAddr};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    // Načti databázovou URL
    let db_url = env::var("DATABASE_URL").expect("Chybí DATABASE_URL v .env");

    // Parse db_url přes Url kvůli username/heslu/portu atd.
    let parsed_url = Url::parse(&db_url).expect("Neplatná DATABASE_URL");
    let options = PgConnectOptions::from_url(&parsed_url)
        .expect("Nelze vytvořit PgConnectOptions")
        .disable_statement_logging()
        .statement_cache_capacity(0); // <-- vypínáme cache

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    db::inicializuj_db(&db_pool).await?;

    let app = Router::new()
        .route("/", get(|| async { "API SERVER BĚŽÍ" }))
        .route("/register", post(handlers::register_user))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("✅ Server běží na adrese: {}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
