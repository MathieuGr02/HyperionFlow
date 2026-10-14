#[derive(Debug)]
pub struct Bool(pub bool);

impl From<&bool> for Bool {
    fn from(value: &bool) -> Self {
        Bool(value.clone())
    }
}
