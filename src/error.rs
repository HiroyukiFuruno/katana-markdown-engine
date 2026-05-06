use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KmeError {
    #[error("failed to read Markdown file `{path}`: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Markdown source is empty")]
    EmptySource,
}
