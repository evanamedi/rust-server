pub mod logging_middleware;

use crate::db::DbPool;
use crate::middlewares::logging_middleware::LoggingMiddleware;
use std::sync::Arc;

pub trait Middleware: MiddlewareClone + Send {
    fn handle(&self, request: &mut [u8], response: &mut String);
}

pub trait MiddlewareClone {
    fn clone_box(&self) -> Box<dyn Middleware>;
}

impl<T> MiddlewareClone for T
where
    T: 'static + Middleware + Clone,
{
    fn clone_box(&self) -> Box<dyn Middleware> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Middleware> {
    fn clone(&self) -> Box<dyn Middleware> {
        self.clone_box()
    }
}

pub fn initialize_middlewares(pool: Arc<DbPool>) -> Vec<Box<dyn Middleware>> {
    vec![Box::new(LoggingMiddleware::new(pool))]
}

pub fn execute_middlewares(
    middlewares: &Vec<Box<dyn Middleware>>,
    request: &mut [u8],
    response: &mut String,
) {
    for middleware in middlewares {
        middleware.handle(request, response);
    }
}
