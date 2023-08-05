use std::env::var;

use sea_orm_migration::{sea_orm::{DatabaseConnection, Database}, DbErr};
pub mod migrate_from_entity;

pub struct DB {
    db_name: String,
    db_user: String,
    db_port: String,
    db_host: String,
    db_password: String,
    db_url: Option<String>,
}

impl DB {
    pub fn init() -> Self {
        let remote: bool = match var("REMOTE_DB") {
            Ok(val) => val == "true",
            Err(_) => false,
        };

        if remote {
            let db_url = match var("REMOTE_DB_URL") {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            Self {
                db_name: var("REMOTE_DB_NAME").expect("REMOTE_DB_NAME env var not set"),
                db_user: var("REMOTE_DB_USER").expect("REMOTE_DB_USER env var not set"),
                db_port: var("REMOTE_DB_PORT").expect("REMOTE_DB_PORT env var not set"),
                db_host: var("REMOTE_DB_HOST").expect("REMOTE_DB_HOST env var not set"),
                db_password: var("REMOTE_DB_PASSWORD").expect("REMOTE_DB_PASSWORD env var not set"),
                db_url,
            }
        } else {
            Self {
                db_name: var("DB_NAME").expect("DB_NAME env var not set"),
                db_user: var("DB_USER").expect("DB_USER env var not set"),
                db_port: var("DB_PORT").expect("DB_PORT env var not set"),
                db_host: var("DB_HOST").expect("DB_HOST env var not set"),
                db_password: var("DB_PASSWORD").expect("DB_PASSWORD env var not set"),
                db_url: None,
            }
        }
    }

    pub fn url(&self) -> String {
        match &self.db_url {
            Some(val) => val.to_string(),
            None => {
                let name = &self.db_name;
                let user = &self.db_user;
                let password = &self.db_password;
                let host = &self.db_host;
                let port = &self.db_port;
                format!("postgresql://{user}:{password}@{host}:{port}/{name}")
            }
        }
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        let url = self.url();
        Database::connect(url).await
    }
}
