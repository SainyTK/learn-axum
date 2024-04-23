mod routes;
mod utils;

use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use routes::create_routes;
use utils::database::connect_db;

pub async fn run() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    connect_db(database_uri).await;

    let app = create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}