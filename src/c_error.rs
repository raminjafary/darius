use std::error::Error;
use std::fmt;

// type Result<T> = std::result::Result<T, LogError>;

#[derive(Debug, Clone)]
pub struct LogError {
    reason: ErrorReason,
}

#[derive(Debug, Clone)]
pub struct ErrorReason;

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LogError is here!")
    }
}

impl Error for LogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.reason)
    }
}

impl fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorReason is here!")
    }
}

impl Error for ErrorReason {}

fn throw() -> Result<(), LogError> {
    Err(LogError {
        reason: ErrorReason,
    })
}

pub fn throw_or<S: AsRef<str>>(_s: S) -> Result<(), LogError> {
    match throw() {
        Err(e) => {
            eprintln!("e: {}", e);
            eprintln!("Caused by: {}", e.source().unwrap());
        }

        Ok(_s) => _s,
    }
    Ok(())
}
