use std::path::PathBuf;

#[allow(dead_code)]
pub struct WAL {
    wal_file: PathBuf,
}

impl WAL {
    pub fn new(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        let path = path.into();
        let name = name.into();
        let path = path.join(name);
        WAL { wal_file: path }
    }
}
