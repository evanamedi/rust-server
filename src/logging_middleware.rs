use crate::middleware::Middleware;

#[derive(Clone)]
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut [u8], _response: &mut String) {
        println!("Received request: {}", String::from_utf8_lossy(request));
    }
}
