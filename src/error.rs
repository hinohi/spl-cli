#[derive(Debug)]
pub enum Error {
    Terminal,
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Terminal => f.write_str("Terminal"),
            Error::Io(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
