#[derive(Debug)]
pub struct Float(f64);

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value)
    }
}

impl From<&f64> for Float {
    fn from(value: &f64) -> Self {
        Float(value.clone())
    }
}
