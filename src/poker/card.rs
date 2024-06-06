use std::fmt;

use serde::de;

use crate::error::{Error, Result};

/// Card suits.
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
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

/// Card rank or rank.
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub enum Rank {
    Two,
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

/// Constant of all ranks.
const RANKS: [Rank; 13] = [
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

impl Rank {
    pub const fn ranks() -> [Self; 13] {
        RANKS
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Rank::Ace),
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' => Some(Rank::Ten),
            'J' => Some(Rank::Jack),
            'Q' => Some(Rank::Queen),
            'K' => Some(Rank::King),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
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
        }
    }

    pub fn from_int(i: i8) -> Option<Self> {
        match i {
            14 => Some(Rank::Ace),
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

    pub fn as_int(&self) -> i8 {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    pub fn gap(&self, other: &Self) -> i8 {
        (self.as_int() - other.as_int()).abs()
    }

    pub fn gap_with_ace(&self) -> i8 {
        Rank::Ace.as_int() - self.as_int()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn try_from_str(str: &str) -> Result<Self> {
        let mut chars = str.chars();
        let suit_char = chars.next().ok_or(Error::UnexpectedCardChar)?;
        let rank_char = chars.next().ok_or(Error::UnexpectedCardChar)?;
        Ok(Self {
            suit: Suit::from_char(suit_char).ok_or(Error::UnexpectedCardChar)?,
            rank: Rank::from_char(rank_char).ok_or(Error::UnexpectedCardChar)?,
        })
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit.as_icon_char(), self.rank.as_char())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}{}", self.suit.as_char(), self.rank.as_char());
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

            fn visit_str<E>(self, rank: &str) -> Result<Card, E>
            where
                E: serde::de::Error,
            {
                if rank.len() != 2 {
                    return Err(serde::de::Error::invalid_length(rank.len(), &self));
                }

                let suit_char = rank.chars().next().unwrap();
                let rank_char = rank.chars().nth(1).unwrap();

                let rank = Rank::from_char(rank_char).ok_or_else(|| {
                    de::Error::unknown_variant(
                        &rank_char.to_string(),
                        &[
                            "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K",
                        ],
                    )
                })?;
                let suit = Suit::from_char(suit_char).ok_or_else(|| {
                    de::Error::unknown_variant(&suit_char.to_string(), &["S", "H", "D", "C"])
                })?;

                Ok(Card { suit, rank })
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
            rank: Rank::Ace,
        };
        assert_eq!(expected, Card::try_from_str("SA").unwrap())
    }

    #[test]
    fn test_rank_cmp() {
        assert!(Rank::Two < Rank::Ace);
        assert!(Rank::King < Rank::Ace);
        assert_eq!(Rank::Two, Rank::Two);
    }

    #[test]
    fn test_suit_cmp() {
        assert!(Suit::Club < Suit::Diamond);
        assert!(Suit::Heart < Suit::Spade);
        assert_eq!(Suit::Diamond, Suit::Diamond);
    }
}
