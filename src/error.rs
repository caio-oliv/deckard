use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ParseSuitError;

impl ParseSuitError {
    pub const fn as_str(&self) -> &'static str {
        "error parsing card suit"
    }
}

impl fmt::Display for ParseSuitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::error::Error for ParseSuitError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ParseRankError;

impl ParseRankError {
    pub const fn as_str(&self) -> &'static str {
        "error parsing card rank"
    }
}

impl fmt::Display for ParseRankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::error::Error for ParseRankError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseCardError {
    Wild,
    Length,
    Suit(ParseSuitError),
    Rank(ParseRankError),
}

impl ParseCardError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Wild => "error parsing wild card",
            Self::Length => "error parsing card with invalid length",
            Self::Suit(err) => err.as_str(),
            Self::Rank(err) => err.as_str(),
        }
    }
}

impl From<ParseSuitError> for ParseCardError {
    fn from(value: ParseSuitError) -> Self {
        Self::Suit(value)
    }
}

impl From<ParseRankError> for ParseCardError {
    fn from(value: ParseRankError) -> Self {
        Self::Rank(value)
    }
}

impl fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::error::Error for ParseCardError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Wild => None,
            Self::Length => None,
            Self::Suit(error) => Some(error),
            Self::Rank(error) => Some(error),
        }
    }
}
