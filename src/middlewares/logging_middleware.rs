use crate::db::DbPool;
use crate::middlewares::Middleware;
use r2d2_sqlite::rusqlite::params;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoggingMiddleware {
    pool: Arc<DbPool>,
}

impl LoggingMiddleware {
    pub fn new(pool: Arc<DbPool>) -> Self {
        LoggingMiddleware { pool }
    }
}

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut [u8], _response: &mut String) {
        let request_str = String::from_utf8_lossy(request);
        let (method, path) = if let Some(space_idx) = request_str.find(' ') {
            let (method, rest) = request_str.split_at(space_idx);
            let path = rest.trim_start().split(' ').next().unwrap_or("");
            (method, path)
        } else {
            ("UNKNOWN", "UNKNOWN")
        };

        let conn = self.pool.get().expect("Failed to get DB connection");
        conn.execute(
            "INSERT INTO request_logs (method, path) VALUES (?1, ?2)",
            params![method, path],
        )
        .expect("Failed to insert log");

        println!("Logged request: {} {}", method, path);
    }
}
