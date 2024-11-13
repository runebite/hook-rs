use thiserror::Error;

#[derive(Debug, Error)]
pub enum HookError {
    #[error("Unknown error has occurred!")]
    Unknown,
}