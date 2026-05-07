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

impl Card {
    pub const ACE_CLUB: Card = Card::new(Rank::Ace, Suit::Club);
    pub const TWO_CLUB: Card = Card::new(Rank::Two, Suit::Club);
    pub const THREE_CLUB: Card = Card::new(Rank::Three, Suit::Club);
    pub const FOUR_CLUB: Card = Card::new(Rank::Four, Suit::Club);
    pub const FIVE_CLUB: Card = Card::new(Rank::Five, Suit::Club);
    pub const SIX_CLUB: Card = Card::new(Rank::Six, Suit::Club);
    pub const SEVEN_CLUB: Card = Card::new(Rank::Seven, Suit::Club);
    pub const EIGHT_CLUB: Card = Card::new(Rank::Eight, Suit::Club);
    pub const NINE_CLUB: Card = Card::new(Rank::Nine, Suit::Club);
    pub const TEN_CLUB: Card = Card::new(Rank::Ten, Suit::Club);
    pub const JACK_CLUB: Card = Card::new(Rank::Jack, Suit::Club);
    pub const QUEEN_CLUB: Card = Card::new(Rank::Queen, Suit::Club);
    pub const KING_CLUB: Card = Card::new(Rank::King, Suit::Club);
    pub const JOCKER_CLUB: Card = Card::new(Rank::Jocker, Suit::Club);

    pub const ACE_DIAMOND: Card = Card::new(Rank::Ace, Suit::Diamond);
    pub const TWO_DIAMOND: Card = Card::new(Rank::Two, Suit::Diamond);
    pub const THREE_DIAMOND: Card = Card::new(Rank::Three, Suit::Diamond);
    pub const FOUR_DIAMOND: Card = Card::new(Rank::Four, Suit::Diamond);
    pub const FIVE_DIAMOND: Card = Card::new(Rank::Five, Suit::Diamond);
    pub const SIX_DIAMOND: Card = Card::new(Rank::Six, Suit::Diamond);
    pub const SEVEN_DIAMOND: Card = Card::new(Rank::Seven, Suit::Diamond);
    pub const EIGHT_DIAMOND: Card = Card::new(Rank::Eight, Suit::Diamond);
    pub const NINE_DIAMOND: Card = Card::new(Rank::Nine, Suit::Diamond);
    pub const TEN_DIAMOND: Card = Card::new(Rank::Ten, Suit::Diamond);
    pub const JACK_DIAMOND: Card = Card::new(Rank::Jack, Suit::Diamond);
    pub const QUEEN_DIAMOND: Card = Card::new(Rank::Queen, Suit::Diamond);
    pub const KING_DIAMOND: Card = Card::new(Rank::King, Suit::Diamond);
    pub const JOCKER_DIAMOND: Card = Card::new(Rank::Jocker, Suit::Diamond);

    pub const ACE_HEART: Card = Card::new(Rank::Ace, Suit::Heart);
    pub const TWO_HEART: Card = Card::new(Rank::Two, Suit::Heart);
    pub const THREE_HEART: Card = Card::new(Rank::Three, Suit::Heart);
    pub const FOUR_HEART: Card = Card::new(Rank::Four, Suit::Heart);
    pub const FIVE_HEART: Card = Card::new(Rank::Five, Suit::Heart);
    pub const SIX_HEART: Card = Card::new(Rank::Six, Suit::Heart);
    pub const SEVEN_HEART: Card = Card::new(Rank::Seven, Suit::Heart);
    pub const EIGHT_HEART: Card = Card::new(Rank::Eight, Suit::Heart);
    pub const NINE_HEART: Card = Card::new(Rank::Nine, Suit::Heart);
    pub const TEN_HEART: Card = Card::new(Rank::Ten, Suit::Heart);
    pub const JACK_HEART: Card = Card::new(Rank::Jack, Suit::Heart);
    pub const QUEEN_HEART: Card = Card::new(Rank::Queen, Suit::Heart);
    pub const KING_HEART: Card = Card::new(Rank::King, Suit::Heart);
    pub const JOCKER_HEART: Card = Card::new(Rank::Jocker, Suit::Heart);

    pub const ACE_SPADE: Card = Card::new(Rank::Ace, Suit::Spade);
    pub const TWO_SPADE: Card = Card::new(Rank::Two, Suit::Spade);
    pub const THREE_SPADE: Card = Card::new(Rank::Three, Suit::Spade);
    pub const FOUR_SPADE: Card = Card::new(Rank::Four, Suit::Spade);
    pub const FIVE_SPADE: Card = Card::new(Rank::Five, Suit::Spade);
    pub const SIX_SPADE: Card = Card::new(Rank::Six, Suit::Spade);
    pub const SEVEN_SPADE: Card = Card::new(Rank::Seven, Suit::Spade);
    pub const EIGHT_SPADE: Card = Card::new(Rank::Eight, Suit::Spade);
    pub const NINE_SPADE: Card = Card::new(Rank::Nine, Suit::Spade);
    pub const TEN_SPADE: Card = Card::new(Rank::Ten, Suit::Spade);
    pub const JACK_SPADE: Card = Card::new(Rank::Jack, Suit::Spade);
    pub const QUEEN_SPADE: Card = Card::new(Rank::Queen, Suit::Spade);
    pub const KING_SPADE: Card = Card::new(Rank::King, Suit::Spade);
    pub const JOCKER_SPADE: Card = Card::new(Rank::Jocker, Suit::Spade);

    pub const NEW_DECK_FACE_UP: [Card; 52] = [
        Card::ACE_SPADE,
        Card::TWO_SPADE,
        Card::THREE_SPADE,
        Card::FOUR_SPADE,
        Card::FIVE_SPADE,
        Card::SIX_SPADE,
        Card::SEVEN_SPADE,
        Card::EIGHT_SPADE,
        Card::NINE_SPADE,
        Card::TEN_SPADE,
        Card::JACK_SPADE,
        Card::QUEEN_SPADE,
        Card::KING_SPADE,
        Card::ACE_DIAMOND,
        Card::TWO_DIAMOND,
        Card::THREE_DIAMOND,
        Card::FOUR_DIAMOND,
        Card::FIVE_DIAMOND,
        Card::SIX_DIAMOND,
        Card::SEVEN_DIAMOND,
        Card::EIGHT_DIAMOND,
        Card::NINE_DIAMOND,
        Card::TEN_DIAMOND,
        Card::JACK_DIAMOND,
        Card::QUEEN_DIAMOND,
        Card::KING_DIAMOND,
        Card::KING_CLUB,
        Card::QUEEN_CLUB,
        Card::JACK_CLUB,
        Card::TEN_CLUB,
        Card::NINE_CLUB,
        Card::EIGHT_CLUB,
        Card::SEVEN_CLUB,
        Card::SIX_CLUB,
        Card::FIVE_CLUB,
        Card::FOUR_CLUB,
        Card::THREE_CLUB,
        Card::TWO_CLUB,
        Card::ACE_CLUB,
        Card::KING_HEART,
        Card::QUEEN_HEART,
        Card::JACK_HEART,
        Card::TEN_HEART,
        Card::NINE_HEART,
        Card::EIGHT_HEART,
        Card::SEVEN_HEART,
        Card::SIX_HEART,
        Card::FIVE_HEART,
        Card::FOUR_HEART,
        Card::THREE_HEART,
        Card::TWO_HEART,
        Card::ACE_HEART,
    ];

    pub const NEW_DECK_FACE_DOWN: [Card; 52] = const {
        let mut deck = Card::NEW_DECK_FACE_UP;
        deck.reverse();
        deck
    };

    pub const STANDARD_52: [Card; 52] = const {
        let mut deck = [Card::ACE_SPADE; 52];
        let mut idx: usize = 0;

        // for suit in Suit::ALL {
        //     for rank in Rank::STANDARD {
        //         deck[idx] = Card::new(rank, suit);
        //         idx += 1;
        //     }
        // }
        while idx < 52 {
            deck[idx] = Card::new(
                Rank::STANDARD[idx % Rank::STANDARD_COUNT],
                Suit::ALL[idx / Rank::STANDARD_COUNT],
            );
            idx += 1;
        }

        deck
    };

    pub const STANDARD_52_REVERSE: [Card; 52] = const {
        let mut deck = Card::STANDARD_52;
        deck.reverse();
        deck
    };
}
