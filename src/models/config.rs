use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Config {
    pub root_dir: Option<PathBuf>,
}
