use std::{
    fmt,
    ops::{Index, RangeFrom, RangeFull, RangeTo},
    slice::Iter,
};

use crate::error::Result;

use super::card::Card;

#[derive(Debug)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new_with_cards(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn new_from_strs(strs: &[&str]) -> Result<Self> {
        let mut cards = Vec::new();
        for s in strs {
            let card = Card::try_from_str(s)?;
            cards.push(card)
        }
        Ok(Self(cards))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, c: Card) -> &mut Self {
        self.0.push(c);
        self
    }

    pub fn remove(&mut self, len: usize) -> &mut Self {
        self.0.remove(len);
        self
    }

    pub fn truncate(&mut self, len: usize) -> &mut Self {
        self.0.truncate(len);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<Card> {
        self.0.iter()
    }
}

/// Allow indexing into the hand.
impl Index<usize> for Hand {
    type Output = Card;
    fn index(&self, index: usize) -> &Card {
        &self.0[index]
    }
}

/// Allow the index to get refernce to every card.
impl Index<RangeFull> for Hand {
    type Output = [Card];
    fn index(&self, range: RangeFull) -> &[Card] {
        &self.0[range]
    }
}

impl Index<RangeTo<usize>> for Hand {
    type Output = [Card];
    fn index(&self, index: RangeTo<usize>) -> &[Card] {
        &self.0[index]
    }
}

impl Index<RangeFrom<usize>> for Hand {
    type Output = [Card];
    fn index(&self, index: RangeFrom<usize>) -> &[Card] {
        &self.0[index]
    }
}

impl Extend<Card> for Hand {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, card) in self.iter().enumerate() {
            if i > 0 {
                if i % 10 == 0 {
                    writeln!(f)?;
                } else {
                    write!(f, ", ")?;
                }
            }
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_hand() {
        let hand = vec!["SA", "ht", "D9", "c2"];
        assert!(Hand::new_from_strs(&hand).is_ok());

        let hand2 = vec!["sa", "sx"];
        assert!(Hand::new_from_strs(&hand2).is_err());
    }
}
