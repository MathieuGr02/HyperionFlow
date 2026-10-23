use std::sync::{Arc, atomic::AtomicBool};

pub trait Runnable: Send + 'static {
    type Output: Send;
    fn run(&mut self, shutdown: Arc<AtomicBool>) -> Self::Output;
}
