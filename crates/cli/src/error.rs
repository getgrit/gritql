/// A good error isn't really an error at all.
/// It means Grit is functioning correctly, but should return a non-zero exit code.
/// This is mainly used for `grit check`, where non-zero exit codes are used to signal that there are issues.
#[derive(Debug)]
pub struct GoodError {
    pub message: Option<String>,
}

impl GoodError {
    pub fn new() -> GoodError {
        GoodError { message: None }
    }

    pub fn new_with_message(message: String) -> GoodError {
        GoodError {
            message: Some(message),
        }
    }
}

impl std::fmt::Display for GoodError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}", message),
            None => write!(f, "Grit behaved as expected, but exited with an error."),
        }
    }
}

impl std::error::Error for GoodError {}
