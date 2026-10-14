pub trait QueryMapper {
    type Input;
    type Output;
    type Error;

    fn map(&self, query: Self::Input) -> Result<Self::Output, Self::Error>;
}
