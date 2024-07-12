#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("IO {0}")]
    IO(#[from] std::io::Error),
}
