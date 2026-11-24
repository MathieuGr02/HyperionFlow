pub trait DatabaseType {
    const NAME: &str;
    const PLACEHOLDER: &str;
}
