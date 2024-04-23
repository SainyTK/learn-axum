use sea_orm::Database;

pub async fn connect_db(database_uri: &str) {
    let database = Database::connect(database_uri).await;
}