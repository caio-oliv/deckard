mod impls;

#[cfg(test)]
mod tests;

use crate::card::Card;

pub trait CardCompare: PartialEq + PartialOrd + From<Card> + Into<Card> {}
