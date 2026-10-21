use std::sync::{Arc, atomic::AtomicBool};

pub trait Component {
    type Output;
    type Error;

    fn main_loop(&mut self, shutdown: Arc<AtomicBool>) -> Result<Self::Output, Self::Error>;
}
