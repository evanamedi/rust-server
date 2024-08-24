use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db() -> Result<Arc<DbPool>, Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file("middleware.db");
    let pool = Pool::new(manager)?;
    let conn = pool.get()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS request_logs (
                id INTEGER PRIMARY KEY,
                method TEXT NOT NULL,
                path TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
    )",
        [],
    )?;

    Ok(Arc::new(pool))
}
