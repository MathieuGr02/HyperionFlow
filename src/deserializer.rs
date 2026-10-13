pub trait Deserializer {
    type Input;
    type Output;

    fn deserialize(&self, item: Self::Input) -> Self::Output;
}
