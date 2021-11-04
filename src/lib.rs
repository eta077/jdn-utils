use std::error::Error;
use std::fmt;

/// An enumeration of the errors that can occur while ensuring a String is normalized.
#[derive(Debug)]
pub enum NormalizationError {
    /// Indicates that the proposed value is empty.
    Empty(&'static str),
    /// Indicates that the proposed value exceeds the maximum length.
    TooLong(&'static str, usize),
    /// Indicates that the proposed value contains one or more restricted character.
    InvalidCharacter(&'static str, &'static str),
}

impl fmt::Display for NormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NormalizationError::Empty(val) => write!(f, "{} cannot be empty.", val),
            NormalizationError::TooLong(val, max) => {
                write!(f, "{} cannot be longer than {} characters.", val, max)
            }
            NormalizationError::InvalidCharacter(val, restricted) => {
                write!(f, "{} cannot contain '{}'", val, restricted)
            }
        }
    }
}

impl Error for NormalizationError {}

/// A container for a String that enforces a maximum length and restricted characters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NormalizedString(String);

impl NormalizedString {
    /// Verifies that the proposed value meets the requirements for a NormalizedString with the following default
    /// values:
    /// Maximum Length: 255
    /// Restricted Characters: [';']
    pub fn default_verify(
        value: String,
        value_name: &'static str,
    ) -> Result<NormalizedString, NormalizationError> {
        NormalizedString::verify(value, value_name, 255, ";")
    }

    /// Verifies that the proposed value meets the requirements for a NormalizedString with the given parameters.
    pub fn verify(
        value: String,
        value_name: &'static str,
        max_len: usize,
        restricted_chars: &'static str,
    ) -> Result<NormalizedString, NormalizationError> {
        if value.is_empty() {
            return Result::Err(NormalizationError::Empty(value_name));
        }
        if value.len() > max_len {
            return Result::Err(NormalizationError::TooLong(value_name, max_len));
        }
        for c in restricted_chars.chars() {
            if value.contains(c) {
                return Result::Err(NormalizationError::InvalidCharacter(
                    value_name,
                    restricted_chars,
                ));
            }
        }
        Result::Ok(NormalizedString(value))
    }

    /// Returns a reference to the wrapped value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<String> for NormalizedString {
    fn into(self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for NormalizedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
