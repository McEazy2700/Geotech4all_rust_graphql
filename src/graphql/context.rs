use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct AppContext {
    pub db: DatabaseConnection
}
