#[derive(Debug)]
pub struct Text(String);

impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Text(value.to_owned())
    }
}
