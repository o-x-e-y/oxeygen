use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Couldn't parse the thing you sent lmfao")]
    ParseError,
}

pub fn cli() -> Result<(), CliError> {
    Ok(())
}
