use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://data/app.db".to_string());

        let pool = SqlitePool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}