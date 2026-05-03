mod impls;

#[cfg(test)]
mod tests;

use core::num::NonZeroU8;

use crate::error::{ParseCardError, ParseRankError, ParseSuitError};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Suit {
    Club = 0b0000_0000,
    Diamond = 0b0001_0000,
    Heart = 0b0010_0000,
    Spade = 0b0011_0000,
}

impl Suit {
    const MASK: u8 = 0b0011_0000;

    pub const MIN: Suit = Suit::Club;
    pub const MAX: Suit = Suit::Spade;

    pub const COUNT: usize = 4;

    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    pub const RED: [Suit; 2] = [Suit::Diamond, Suit::Heart];
    pub const BLACK: [Suit; 2] = [Suit::Club, Suit::Spade];

    pub const SYMBOL_UTF8_LEN: usize = 3;

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn as_char(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        }
    }

    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, ParseSuitError> {
        if bytes.len() != Self::SYMBOL_UTF8_LEN {
            return Err(ParseSuitError);
        }
        if bytes == Suit::Club.as_str().as_bytes() {
            return Ok(Suit::Club);
        }
        if bytes == Suit::Diamond.as_str().as_bytes() {
            return Ok(Suit::Diamond);
        }
        if bytes == Suit::Heart.as_str().as_bytes() {
            return Ok(Suit::Heart);
        }
        if bytes == Suit::Spade.as_str().as_bytes() {
            return Ok(Suit::Spade);
        }
        Err(ParseSuitError)
    }

    pub const fn parse_char(c: char) -> Result<Self, ParseSuitError> {
        match c {
            '♣' => Ok(Suit::Club),
            '♦' => Ok(Suit::Diamond),
            '♥' => Ok(Suit::Heart),
            '♠' => Ok(Suit::Spade),
            _ => Err(ParseSuitError),
        }
    }

    pub fn parse_str(s: &str) -> Result<Self, ParseSuitError> {
        Self::parse_bytes(s.as_bytes())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Jocker = 0b1111,
}

impl Rank {
    const MASK: u8 = 0b0000_1111;

    pub const MIN: Rank = Rank::Ace;
    pub const MAX: Rank = Rank::Jocker;

    pub const STANDARD_COUNT: usize = 13;
    pub const ALL_COUNT: usize = 14;

    pub const SYMBOL_UTF8_LEN: usize = 1;

    pub const STANDARD: [Rank; 13] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];
    pub const STANDARD_REVERSE: [Rank; 13] = const {
        let mut stack = Self::STANDARD;
        stack.reverse();
        stack
    };

    pub const ALL: [Rank; 14] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Jocker,
    ];
    pub const ALL_REVERSE: [Rank; 14] = const {
        let mut stack = Self::ALL;
        stack.reverse();
        stack
    };

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Rank::Ace),
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            9 => Some(Rank::Nine),
            10 => Some(Rank::Ten),
            11 => Some(Rank::Jack),
            12 => Some(Rank::Queen),
            13 => Some(Rank::King),
            _ => None,
        }
    }

    pub const fn is_royalty(self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub const fn is_numbered(self) -> bool {
        matches!(
            self,
            Rank::Two
                | Rank::Three
                | Rank::Four
                | Rank::Five
                | Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten
        )
    }

    pub const fn pip_count(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack | Rank::Queen | Rank::King | Rank::Jocker => 0,
        }
    }

    pub const fn has_pip(self) -> bool {
        self.pip_count() != 0
    }

    pub const fn as_char(self) -> char {
        match self {
            Rank::Ace => 'A',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Jocker => 'U',
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Jocker => "U",
        }
    }

    pub const fn parse_byte(b: u8) -> Result<Self, ParseRankError> {
        match b {
            b'A' => Ok(Rank::Ace),
            b'2' => Ok(Rank::Two),
            b'3' => Ok(Rank::Three),
            b'4' => Ok(Rank::Four),
            b'5' => Ok(Rank::Five),
            b'6' => Ok(Rank::Six),
            b'7' => Ok(Rank::Seven),
            b'8' => Ok(Rank::Eight),
            b'9' => Ok(Rank::Nine),
            b'T' => Ok(Rank::Ten),
            b'J' => Ok(Rank::Jack),
            b'Q' => Ok(Rank::Queen),
            b'K' => Ok(Rank::King),
            b'U' => Ok(Rank::Jocker),
            _ => Err(ParseRankError),
        }
    }

    pub const fn parse_char(c: char) -> Result<Self, ParseRankError> {
        if c.len_utf8() != Self::SYMBOL_UTF8_LEN {
            return Err(ParseRankError);
        }
        Self::parse_byte(c as u8)
    }

    pub const fn parse_str(s: &str) -> Result<Self, ParseRankError> {
        let bytes = s.as_bytes();
        if bytes.len() != Self::SYMBOL_UTF8_LEN {
            return Err(ParseRankError);
        }
        Self::parse_byte(bytes[0])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum CardKind {
    #[default]
    Normal = 0,
    Wild = Card::WILD_MASK,
}

impl CardKind {
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(NonZeroU8);

impl Card {
    const WILD_MASK: u8 = 0b1000_0000;

    pub const MIN: Card = Card::new(Rank::MIN, Suit::MIN);
    pub const MAX: Card = Card::wild(Rank::MAX, Suit::MAX);

    pub const STANDARD_COUNT: usize = 52;

    pub const WILD: Card = Card::MAX;

    pub const WILD_SYMBOL: u8 = b'*';
    pub const WILD_SYMBOL_CHAR: char = '*';
    pub const WILD_SYMBOL_STR: &str = "*";

    pub const UTF8_MIN_LEN: usize = 1;
    pub const UTF8_MAX_LEN: usize = Rank::SYMBOL_UTF8_LEN + Suit::SYMBOL_UTF8_LEN + 1;

    pub const fn wild(rank: Rank, suit: Suit) -> Self {
        Self(
            // SAFETY: (WILD_MASK | rank | suit) always produces a non-zero u8
            unsafe { NonZeroU8::new_unchecked(Self::WILD_MASK | rank.to_u8() | suit.to_u8()) },
        )
    }

    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self(
            // SAFETY: (rank | suit) always produces a non-zero u8
            unsafe { NonZeroU8::new_unchecked(rank.to_u8() | suit.to_u8()) },
        )
    }

    pub const fn new_kind(kind: CardKind, rank: Rank, suit: Suit) -> Self {
        Self(
            // SAFETY: (kind | rank | suit) always produces a non-zero u8
            unsafe { NonZeroU8::new_unchecked(kind.to_u8() | rank.to_u8() | suit.to_u8()) },
        )
    }

    pub const fn suit(self) -> Suit {
        // SAFETY: (Suit::MASK & u8) always get a valid Suit variant
        unsafe { core::mem::transmute(self.0.get() & Suit::MASK) }
    }

    pub const fn rank(self) -> Rank {
        // SAFETY: Card is composed by a (rank | suit), so (Rank::MASK & card) always
        // get a valid Rank variant
        unsafe { core::mem::transmute(self.0.get() & Rank::MASK) }
    }

    pub const fn kind(self) -> CardKind {
        // SAFETY: WILD_MASK only contains the left most bit set, so any bitand
        // operation with u8 will produce either 0 xor WILD_MASK, which is the
        // bit pattern of CardKind
        unsafe { core::mem::transmute(self.0.get() & Card::WILD_MASK) }
    }

    pub const fn is_wild(self) -> bool {
        (self.0.get() & Card::WILD_MASK) != 0
    }

    pub const fn is_normal(self) -> bool {
        (self.0.get() & Card::WILD_MASK) == 0
    }

    /// Converts self to a will card
    pub const fn to_wild(self) -> Self {
        Self(
            // SAFETY: (WILD_MASK | u8) always produces a non-zero u8
            unsafe { NonZeroU8::new_unchecked(Self::WILD_MASK | self.0.get()) },
        )
    }

    pub const fn to_normal(self) -> Self {
        Self(
            // SAFETY: (u8 & !WILD_MASK) always produces a non-zero u8
            unsafe { NonZeroU8::new_unchecked(self.0.get() & !Self::WILD_MASK) },
        )
    }

    pub const fn to_kind(self, kind: CardKind) -> Self {
        match kind {
            CardKind::Normal => self.to_normal(),
            CardKind::Wild => self.to_wild(),
        }
    }

    pub const fn to_rank(self, rank: Rank) -> Self {
        // SAFETY: ((self & !RANK_MASK) | rank) always produces a non-zero u8
        Self(unsafe { NonZeroU8::new_unchecked((self.0.get() & !Rank::MASK) | rank.to_u8()) })
    }

    pub const fn to_suit(self, suit: Suit) -> Self {
        // SAFETY: ((self & !SUIT_MASK) | suit) always produces a non-zero u8
        Self(unsafe { NonZeroU8::new_unchecked((self.0.get() & !Suit::MASK) | suit.to_u8()) })
    }

    pub const fn to_u8(self) -> NonZeroU8 {
        self.0
    }

    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, ParseCardError> {
        if bytes.len() < Card::UTF8_MIN_LEN || bytes.len() > Card::UTF8_MAX_LEN {
            return Err(ParseCardError::Length);
        }

        let (kind, remain) = if bytes[0] == Card::WILD_SYMBOL {
            (CardKind::Wild, &bytes[1..])
        } else {
            (CardKind::Normal, bytes)
        };

        if kind == CardKind::Wild && remain.is_empty() {
            return Ok(Card::WILD);
        }

        let rank = Rank::parse_byte(remain[0])?;
        let suit = Suit::parse_bytes(&remain[1..])?;

        Ok(Self::new_kind(kind, rank, suit))
    }

    pub fn parse_str(s: &str) -> Result<Self, ParseCardError> {
        Self::parse_bytes(s.as_bytes())
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
