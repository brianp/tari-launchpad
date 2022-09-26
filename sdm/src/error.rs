use thiserror::Error;

#[derive(Debug, Error)]
#[error("Can't parse value: {0}")]
pub struct ParseError(pub String);
