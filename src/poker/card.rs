use std::fmt;

use serde::de;

use crate::error::{Error, Result};

/// Card suits.
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub enum Suit {
    /// Spades
    Spade = 3,
    /// Hearts
    Heart = 2,
    /// Diamonds
    Diamond = 1,
    /// Clubs
    Club = 0,
}

/// All of the Suits
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

impl Suit {
    pub const fn suits() -> [Self; 4] {
        SUITS
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'S' => Some(Suit::Spade),
            'H' => Some(Suit::Heart),
            'D' => Some(Suit::Diamond),
            'C' => Some(Suit::Club),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Suit::Spade => 'S',
            Suit::Heart => 'H',
            Suit::Diamond => 'D',
            Suit::Club => 'C',
        }
    }

    fn as_icon_char(&self) -> char {
        match self {
            Suit::Spade => '♠',
            Suit::Heart => '♥',
            Suit::Diamond => '♦',
            Suit::Club => '♣',
        }
    }
}

/// Card value or value.
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub enum Value {
    /// Ace Value 1 or 14
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Constant of all values.
const RANKS: [Value; 13] = [
    Value::Ace,
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
];

impl Value {
    pub const fn values() -> [Self; 13] {
        RANKS
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Value::Ace),
            '2' => Some(Value::Two),
            '3' => Some(Value::Three),
            '4' => Some(Value::Four),
            '5' => Some(Value::Five),
            '6' => Some(Value::Six),
            '7' => Some(Value::Seven),
            '8' => Some(Value::Eight),
            '9' => Some(Value::Nine),
            'T' => Some(Value::Ten),
            'J' => Some(Value::Jack),
            'Q' => Some(Value::Queen),
            'K' => Some(Value::King),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Value::Ace => 'A',
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub struct Card {
    /// The suit of this card.
    pub suit: Suit,
    /// The face value of this card.
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }

    pub fn try_from_str(str: &str) -> Result<Self> {
        let mut chars = str.chars();
        let suit_char = chars.next().ok_or(Error::UnexpectedCardChar)?;
        let value_char = chars.next().ok_or(Error::UnexpectedCardChar)?;
        Ok(Self {
            suit: Suit::from_char(suit_char).ok_or(Error::UnexpectedCardChar)?,
            value: Value::from_char(value_char).ok_or(Error::UnexpectedCardChar)?,
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit.as_icon_char(), self.value.as_char())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}{}", self.suit.as_char(), self.value.as_char());
        serializer.serialize_str(&s)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CardVisitor;

        impl<'de> serde::de::Visitor<'de> for CardVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a card (e.g., SA, H5)")
            }

            fn visit_str<E>(self, value: &str) -> Result<Card, E>
            where
                E: serde::de::Error,
            {
                if value.len() != 2 {
                    return Err(serde::de::Error::invalid_length(value.len(), &self));
                }

                let suit_char = value.chars().next().unwrap();
                let value_char = value.chars().nth(1).unwrap();

                let value = Value::from_char(value_char).ok_or_else(|| {
                    de::Error::unknown_variant(
                        &value_char.to_string(),
                        &[
                            "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K",
                        ],
                    )
                })?;
                let suit = Suit::from_char(suit_char).ok_or_else(|| {
                    de::Error::unknown_variant(&suit_char.to_string(), &["S", "H", "D", "C"])
                })?;

                Ok(Card { suit, value })
            }
        }

        deserializer.deserialize_str(CardVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_card() {
        let expected = Card {
            suit: Suit::Spade,
            value: Value::Ace,
        };
        assert_eq!(expected, Card::try_from_str("SA").unwrap())
    }

    #[test]
    fn test_value_cmp() {
        assert!(Value::Two < Value::Ace);
        assert!(Value::King < Value::Ace);
        assert_eq!(Value::Two, Value::Two);
    }

    #[test]
    fn test_suit_cmp() {
        assert!(Suit::Club < Suit::Diamond);
        assert!(Suit::Heart < Suit::Spade);
        assert_eq!(Suit::Diamond, Suit::Diamond);
    }
}
