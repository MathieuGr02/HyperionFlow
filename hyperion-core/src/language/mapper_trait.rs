pub trait LanguageMapper {
    type Error: std::fmt::Debug;

    fn map(&self) -> Result<(), Self::Error>;
}
