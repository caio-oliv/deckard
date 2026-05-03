use core::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr, BitXor, Not},
    str::FromStr,
};

use crate::{
    card::{Card, CardKind, Rank, Suit},
    error::{ParseCardError, ParseRankError, ParseSuitError},
};

impl From<Suit> for u8 {
    fn from(value: Suit) -> Self {
        value.to_u8()
    }
}

impl FromStr for Suit {
    type Err = ParseSuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for Suit {
    type Error = ParseSuitError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

impl TryFrom<char> for Suit {
    type Error = ParseSuitError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::parse_char(value)
    }
}

impl TryFrom<&[u8]> for Suit {
    type Error = ParseSuitError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse_bytes(value)
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for Suit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        value.to_u8()
    }
}

impl FromStr for Rank {
    type Err = ParseRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for Rank {
    type Error = ParseRankError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

impl TryFrom<char> for Rank {
    type Error = ParseRankError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::parse_char(value)
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl BitAnd for Suit {
    type Output = u8;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.to_u8().bitand(rhs.to_u8())
    }
}

impl BitOr for Suit {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.to_u8().bitor(rhs.to_u8())
    }
}

impl BitOr<Rank> for Suit {
    type Output = u8;

    fn bitor(self, rhs: Rank) -> Self::Output {
        self.to_u8().bitor(rhs.to_u8())
    }
}

impl BitXor for Suit {
    type Output = u8;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.to_u8().bitxor(rhs.to_u8())
    }
}

impl Not for Suit {
    type Output = u8;

    fn not(self) -> Self::Output {
        self.to_u8().not()
    }
}

impl BitAnd for Rank {
    type Output = u8;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.to_u8().bitand(rhs.to_u8())
    }
}

impl BitOr for Rank {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.to_u8().bitor(rhs.to_u8())
    }
}

impl BitOr<Suit> for Rank {
    type Output = u8;

    fn bitor(self, rhs: Suit) -> Self::Output {
        self.to_u8().bitor(rhs.to_u8())
    }
}

impl BitXor for Rank {
    type Output = u8;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.to_u8().bitxor(rhs.to_u8())
    }
}

impl Not for Rank {
    type Output = u8;

    fn not(self) -> Self::Output {
        self.to_u8().not()
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

impl TryFrom<&str> for Card {
    type Error = ParseCardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse_str(value)
    }
}

impl From<(Rank, Suit)> for Card {
    fn from(value: (Rank, Suit)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Suit, Rank)> for Card {
    fn from(value: (Suit, Rank)) -> Self {
        Self::new(value.1, value.0)
    }
}

impl From<(CardKind, Rank, Suit)> for Card {
    fn from(value: (CardKind, Rank, Suit)) -> Self {
        Self::new_kind(value.0, value.1, value.2)
    }
}

impl From<(CardKind, Suit, Rank)> for Card {
    fn from(value: (CardKind, Suit, Rank)) -> Self {
        Self::new_kind(value.0, value.2, value.1)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_wild() {
            f.write_str(Self::WILD_SYMBOL_STR)?;
        }
        Display::fmt(&self.rank(), f)?;
        Display::fmt(&self.suit(), f)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self, f)
    }
}
