use crate::error::{Error, Result};
use crate::poker::card::Card;
use crate::poker::hand::Hand;

#[derive(PartialEq, Eq, Debug)]
pub enum HandType {
    Offsuit,
    Suited,
    Paired,
    UnPaired,
}

pub trait HandEvaluator {
    fn cards(&self) -> &[Card];

    fn evaluate(&self) -> Result<HandType> {
        let cards = self.cards();
        if cards.len() != 2 {
            return Err(Error::InvalidHandSize);
        }

        let c1 = &cards[0];
        let c2 = &cards[1];

        if c1.rank() == c2.rank() {
            return Ok(HandType::Paired);
        }

        if c1.suit() == c2.suit() {
            return Ok(HandType::Suited);
        }

        Ok(HandType::Offsuit)
    }
}

/// Implementation for `Hand`
impl HandEvaluator for Hand {
    fn cards(&self) -> &[Card] {
        &self.cards()
    }
}

#[cfg(test)]
mod tests {
    use crate::{holdem::hand_evaluator::HandType, poker::hand::Hand};

    use super::HandEvaluator;

    #[test]
    fn test_suited() {
        let hand1 = Hand::new_from_strs(&["St", "s3"]).unwrap();
        let t = hand1.evaluate().unwrap();
        assert_eq!(HandType::Suited, t)
    }

    #[test]
    fn test_off_suited() {
        let hand1 = Hand::new_from_strs(&["dt", "s3"]).unwrap();
        let t = hand1.evaluate().unwrap();
        assert_eq!(HandType::Offsuit, t)
    }

    #[test]
    fn test_pair() {
        let hand1 = Hand::new_from_strs(&["dt", "st"]).unwrap();
        let t = hand1.evaluate().unwrap();
        assert_eq!(HandType::Paired, t)
    }
}
