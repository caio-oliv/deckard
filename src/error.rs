#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ParseSuitError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ParseRankError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseCardError {
    Wild,
    Length,
    Suit(ParseSuitError),
    Rank(ParseRankError),
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
