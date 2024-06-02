use std::{
    collections::{hash_set::Iter, HashSet},
    fmt,
};

use super::card::{Card, Suit, Value};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Deck {
    cards: HashSet<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: HashSet::new(),
        }
    }

    pub fn insert(&mut self, c: Card) -> bool {
        self.cards.insert(c)
    }

    pub fn remove(&mut self, c: &Card) -> bool {
        self.cards.remove(c)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }

    pub fn iter(&self) -> Iter<Card> {
        self.cards.iter()
    }

    pub fn get_all_cards(&self) -> Vec<Card> {
        self.cards.iter().cloned().collect()
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        if let Some(card) = self.cards.iter().next().cloned() {
            self.cards.remove(&card);
            Some(card)
        } else {
            None
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards: HashSet<Card> = HashSet::new();
        for s in &Suit::suits() {
            for r in &Value::values() {
                cards.insert(Card {
                    suit: s.clone(),
                    value: r.clone(),
                });
            }
        }
        Self { cards }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards_vec: Vec<&Card> = self.cards.iter().collect();
        for (i, card) in cards_vec.iter().enumerate() {
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
    fn test_contains_in() {
        let d = Deck::default();
        assert!(d.contains(&Card {
            suit: Suit::Spade,
            value: Value::Ace,
        }));
    }

    #[test]
    fn test_remove() {
        let mut d = Deck::default();
        let c = Card {
            suit: Suit::Heart,
            value: Value::Queen,
        };
        assert!(d.contains(&c));
        assert!(d.remove(&c));
        assert!(!d.contains(&c));
        assert!(!d.remove(&c));
    }
}
