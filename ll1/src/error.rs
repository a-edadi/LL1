// Define an error enum to capture specific errors.
#[derive(Debug)]
pub enum Ll1Error {
    FirstFollowValidationError(String),
    ParsingTableError(String),
}

impl std::fmt::Display for Ll1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ll1Error::FirstFollowValidationError(msg) => {
                write!(f, "FIRST/FOLLOW Validation Error: {}", msg)
            }
            Ll1Error::ParsingTableError(msg) => {
                write!(f, "Parsing Table Error: {}", msg)
            }
        }
    }
}
