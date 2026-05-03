use core::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use crate::{card::Card, cmp::CardCompare};

impl CardCompare for Card {}

#[derive(Debug, Clone, Copy)]
pub struct WildCardComparator(pub Card);

impl WildCardComparator {
    pub fn is_card_eq(this: Card, other: Card) -> bool {
        (this.is_wild() || other.is_wild()) || this == other
    }
}

impl Deref for WildCardComparator {
    type Target = Card;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WildCardComparator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Card> for WildCardComparator {
    fn from(value: Card) -> Self {
        Self(value)
    }
}

impl From<WildCardComparator> for Card {
    fn from(value: WildCardComparator) -> Self {
        value.0
    }
}

impl PartialEq for WildCardComparator {
    fn eq(&self, other: &Self) -> bool {
        Self::is_card_eq(self.0, other.0)
    }
}

impl CardCompare for WildCardComparator {}

impl PartialOrd for WildCardComparator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if Self::is_card_eq(self.0, other.0) {
            return Some(Ordering::Equal);
        }
        Card::partial_cmp(&self.0, &other.0)
    }
}
