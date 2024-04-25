use dotenvy::dotenv;
use dotenvy_macro::{self, dotenv};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn db_connetion_config() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let db_str = dotenv!("DATABASE_URL");
    match Database::connect(db_str).await {
        Ok(connection) => Ok(connection),
        Err(error) => Err(error),
    }
}
