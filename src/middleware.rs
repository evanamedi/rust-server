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
