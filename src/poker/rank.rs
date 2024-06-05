use super::{card::Card, hand::Hand};

/// All the different possible hand ranks.
/// For each hand rank the u16 corresponds to
/// the strength of the hand in comparison to others
/// of the same rank.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Hash, Debug)]
pub enum Rank {
    /// The lowest rank.
    HighCard(u16),
    OnePair(u16),
    TwoPair(u16),
    ThreeOfAKind(u16),
    Straight(u16),
    Flush(u16),
    FullHouse(u16),
    FourOfAKind(u16),
    StraightFlush(u16),
}

/// usize bits of poker values
const USIZE_BIT: u16 = 16;

/// Bit mask for the wheel for straight (Ace, two, three, four, five)
const WHEEL: u16 = 0b1_0000_0000_1111;

fn rank_straight(value_set: u16) -> Option<u16> {
    let left =
        value_set & (value_set << 1) & (value_set << 2) & (value_set << 3) & (value_set << 4);
    let idx = left.leading_zeros() as u16;
    if idx < USIZE_BIT {
        Some(USIZE_BIT - 4 - idx)
    } else if value_set & WHEEL == WHEEL {
        Some(0)
    } else {
        None
    }
}

fn keep_highest(rank: u16) -> u16 {
    1 << (USIZE_BIT - rank.leading_zeros() as u16 - 1)
}

fn keep_n(rank: u16, to_keep: u16) -> u16 {
    let mut result = rank;
    while result.count_ones() as u16 > to_keep {
        result &= result - 1;
    }
    result
}

fn find_flush(suit_value_sets: &[u16]) -> Option<usize> {
    suit_value_sets.iter().position(|sv| sv.count_ones() >= 5)
}

pub trait HandRanker {
    fn cards(&self) -> &[Card];

    /// Rank the cards to find the best 5 card hand.
    fn rank(&self) -> Rank {
        let (count_to_value, suit_value_sets, value_set) = self.compute_counts();

        if let Some(flush_idx) = find_flush(&suit_value_sets) {
            if let Some(rank) = rank_straight(suit_value_sets[flush_idx]) {
                return Rank::StraightFlush(rank);
            } else {
                let rank = keep_n(suit_value_sets[flush_idx], 5);
                return Rank::Flush(rank);
            }
        }

        if count_to_value[4] != 0 {
            let high = keep_highest(value_set ^ count_to_value[4]);
            return Rank::FourOfAKind(count_to_value[4] << 13 | high);
        }

        if count_to_value[3] != 0 && count_to_value[3].count_ones() == 2 {
            let set = keep_highest(count_to_value[3]);
            let pair = count_to_value[3] ^ set;
            return Rank::FullHouse(set << 13 | pair);
        }

        if count_to_value[3] != 0 && count_to_value[2] != 0 {
            let set = count_to_value[3];
            let pair = keep_highest(count_to_value[2]);
            return Rank::FullHouse(set << 13 | pair);
        }

        if let Some(s_rank) = rank_straight(value_set) {
            return Rank::Straight(s_rank);
        }

        if count_to_value[3] != 0 {
            let low = keep_n(value_set ^ count_to_value[3], 2);
            return Rank::ThreeOfAKind(count_to_value[3] << 13 | low);
        }

        if count_to_value[2].count_ones() >= 2 {
            let pairs = keep_n(count_to_value[2], 2);
            let low = keep_highest(value_set ^ pairs);
            return Rank::TwoPair(pairs << 13 | low);
        }

        if count_to_value[2] == 0 {
            return Rank::HighCard(keep_n(value_set, 5));
        }

        let pair = count_to_value[2];
        let low = keep_n(value_set ^ count_to_value[2], 3);
        Rank::OnePair(pair << 13 | low)
    }

    /// Rank this hand assuming it has exactly 5 cards.
    fn rank_five(&self) -> Rank {
        let (count_to_value, suit_value_sets, value_set) = self.compute_counts();
        let unique_card_count = value_set.count_ones();

        match unique_card_count {
            5 => {
                let is_flush = suit_value_sets.iter().any(|&sv| sv.count_ones() == 5);
                match (rank_straight(value_set), is_flush) {
                    (None, false) => Rank::HighCard(value_set),
                    (Some(rank), false) => Rank::Straight(rank),
                    (None, true) => Rank::Flush(value_set),
                    (Some(rank), true) => Rank::StraightFlush(rank),
                }
            }
            4 => {
                let major_rank = count_to_value[2];
                let minor_rank = value_set ^ major_rank;
                Rank::OnePair(major_rank << 13 | minor_rank)
            }
            3 => {
                if count_to_value[3] != 0 {
                    let major_rank = count_to_value[3];
                    let minor_rank = value_set ^ major_rank;
                    Rank::ThreeOfAKind(major_rank << 13 | minor_rank)
                } else {
                    let major_rank = count_to_value[2];
                    let minor_rank = value_set ^ major_rank;
                    Rank::TwoPair(major_rank << 13 | minor_rank)
                }
            }
            2 => {
                if count_to_value[3] != 0 {
                    let major_rank = count_to_value[3];
                    let minor_rank = value_set ^ major_rank;
                    Rank::FullHouse(major_rank << 13 | minor_rank)
                } else {
                    let major_rank = count_to_value[4];
                    let minor_rank = value_set ^ major_rank;
                    Rank::FourOfAKind(major_rank << 13 | minor_rank)
                }
            }
            _ => unreachable!(),
        }
    }

    /// Compute counts and value sets for ranking.
    fn compute_counts(&self) -> ([u16; 5], [u16; 4], u16) {
        let mut value_to_count: [u8; 13] = [0; 13]; // Number of cards for each value (from 2 to Ace)
        let mut suit_value_sets: [u16; 4] = [0; 4]; // Bitmask representing the presence of each value in each suit
        let mut value_set: u16 = 0; // Bitmask representing the presence of each value
        let mut count_to_value: [u16; 5] = [0; 5]; // Bitmask representing the values that appear a specific number of times (0 to 4)

        for c in self.cards() {
            let v = c.rank().clone() as u8;
            let s = c.suit().clone() as u8;
            value_set |= 1 << v;
            value_to_count[v as usize] += 1;
            suit_value_sets[s as usize] |= 1 << v;

            // // Ace count twice as 1 or 14 for straight
            // match c.value {
            //     super::card::Rank::Ace => {
            //         value_set |= 1 << 1;
            //         suit_value_sets[s as usize] |= 1 << 1;
            //     }
            //     _ => {}
            // }
        }

        for (value, &count) in value_to_count.iter().enumerate() {
            count_to_value[count as usize] |= 1 << value;
        }

        (count_to_value, suit_value_sets, value_set)
    }
}

/// Implementation for `Hand`
impl HandRanker for Hand {
    fn cards(&self) -> &[Card] {
        &self.cards()
    }
}
// // maybe don't need this
// impl HandRanker for Vec<Card> {
//     fn cards(&self) -> &[Card] {
//         &self[..]
//     }
// }

/// Compares the ranks of multiple players and returns the index of the winner(s).
/// If there is a tie, returns the indices of all tied players.
pub fn compare_ranks(ranks: &[Rank]) -> Vec<usize> {
    let mut winners: Vec<usize> = vec![];
    if ranks.is_empty() {
        return winners;
    }

    let mut best_rank = &ranks[0];
    winners.push(0);

    for (i, rank) in ranks.iter().enumerate().skip(1) {
        match rank.cmp(best_rank) {
            std::cmp::Ordering::Greater => {
                best_rank = rank;
                winners.clear();
                winners.push(i);
            }
            std::cmp::Ordering::Equal => {
                winners.push(i);
            }
            _ => {}
        }
    }

    winners
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card;
    use crate::poker::hand::Hand;

    #[test]
    fn test_keep_highest() {
        assert_eq!(0b100, keep_highest(0b111));
    }

    #[test]
    fn test_keep_n() {
        assert_eq!(3, keep_n(0b1111, 3).count_ones());
    }

    #[test]
    fn test_cmp() {
        assert!(Rank::HighCard(0) < Rank::StraightFlush(0));
        assert!(Rank::HighCard(0) < Rank::FourOfAKind(0));
        assert!(Rank::HighCard(0) < Rank::ThreeOfAKind(0));
    }

    #[test]
    fn test_cmp_high() {
        assert!(Rank::HighCard(0) < Rank::HighCard(100));
    }

    #[test]
    fn test_high_card_hand() {
        let hand = Hand::new_from_strs(&["da", "h8", "c9", "ct", "c5"]).unwrap();
        let rank = 1 << card::Rank::Ace as u16
            | 1 << card::Rank::Eight as u16
            | 1 << card::Rank::Nine as u16
            | 1 << card::Rank::Ten as u16
            | 1 << card::Rank::Five as u16;

        assert!(Rank::HighCard(rank) == hand.rank_five());
    }

    #[test]
    fn test_flush() {
        let hand = Hand::new_from_strs(&["da", "d8", "d9", "dt", "d5"]).unwrap();
        println!("hand: {}", hand);
        let rank = 1 << card::Rank::Ace as u16
            | 1 << card::Rank::Eight as u16
            | 1 << card::Rank::Nine as u16
            | 1 << card::Rank::Ten as u16
            | 1 << card::Rank::Five as u16;

        println!("rank: {}", rank);

        let rank_five = hand.rank_five();
        println!("rank_five: {:?}", rank_five);

        assert!(Rank::Flush(rank) == rank_five);
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::new_from_strs(&["da", "ca", "d9", "c9", "s9"]).unwrap();
        let rank = (1 << (card::Rank::Nine as u16)) << 13 | 1 << (card::Rank::Ace as u16);
        assert!(Rank::FullHouse(rank) == hand.rank_five());
    }

    #[test]
    fn test_two_pair() {
        // Make a two pair hand.
        let hand = Hand::new_from_strs(&["da", "ca", "D9", "c9", "st"]).unwrap();
        let rank = (1 << card::Rank::Ace as u16 | 1 << card::Rank::Nine as u16) << 13
            | 1 << card::Rank::Ten as u16;
        assert!(Rank::TwoPair(rank) == hand.rank_five());
    }

    #[test]
    fn test_one_pair() {
        let hand = Hand::new_from_strs(&["da", "ca", "d9", "c8", "st"]).unwrap();
        let rank = (1 << card::Rank::Ace as u16) << 13
            | 1 << card::Rank::Nine as u16
            | 1 << card::Rank::Eight as u16
            | 1 << card::Rank::Ten as u16;

        assert!(Rank::OnePair(rank) == hand.rank_five());
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand::new_from_strs(&["da", "ca", "sa", "ha", "st"]).unwrap();
        assert!(
            Rank::FourOfAKind(
                (1 << (card::Rank::Ace as u16) << 13) | 1 << (card::Rank::Ten as u16)
            ) == hand.rank_five()
        );
    }

    #[test]
    fn test_wheel() {
        let hand = Hand::new_from_strs(&["da", "c2", "s3", "h4", "s5"]).unwrap();
        assert!(Rank::Straight(0) == hand.rank_five());
    }

    #[test]
    fn test_straight() {
        let hand = Hand::new_from_strs(&["c2", "s3", "h4", "s5", "d6"]).unwrap();
        assert!(Rank::Straight(1) == hand.rank_five());
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::new_from_strs(&["c2", "s2", "h2", "s5", "d6"]).unwrap();
        let rank = (1 << (card::Rank::Two as u16)) << 13
            | 1 << (card::Rank::Five as u16)
            | 1 << (card::Rank::Six as u16);
        assert!(Rank::ThreeOfAKind(rank) == hand.rank_five());
    }

    #[test]
    fn test_rank_seven_straight_flush() {
        let h = Hand::new_from_strs(&["da", "dk", "dq", "dj", "dt", "d9", "d8"]).unwrap();
        assert_eq!(Rank::StraightFlush(9), h.rank());
    }

    #[test]
    fn test_rank_seven_straight_flush_wheel() {
        // Make sure that we pick up the wheel straight flush
        // over different straight.
        let h = Hand::new_from_strs(&["d2", "d3", "d4", "d5", "h6", "c7", "da"]).unwrap();
        assert_eq!(Rank::StraightFlush(0), h.rank());
    }
    #[test]
    fn test_rank_seven_straights() {
        let straights = [
            ["h2", "c3", "s4", "d5", "d6", "s6", "hk"],
            ["c3", "s4", "d5", "d6", "h7", "st", "hk"],
            ["s4", "d5", "d6", "h7", "c8", "st", "hk"],
            ["c5", "c6", "h7", "h8", "d9", "ha", "da"],
            ["c6", "c7", "h8", "h9", "st", "ck", "s6"],
            ["c7", "h8", "h9", "st", "ck", "s6", "hj"],
            ["h8", "h9", "st", "cq", "s6", "hj", "sa"],
            ["h9", "st", "cq", "s6", "hj", "sk", "ck"],
            ["st", "cq", "s6", "hj", "sk", "ca", "h5"],
        ];
        for (idx, s) in straights.iter().enumerate() {
            assert_eq!(
                Rank::Straight(idx as u16 + 1),
                Hand::new_from_strs(s).unwrap().rank()
            );
        }
    }

    #[test]
    fn test_rank_seven_find_best_with_wheel() {
        let h = Hand::new_from_strs(&["d6", "dk", "da", "d2", "d5", "d4", "d3"]).unwrap();
        assert_eq!(Rank::StraightFlush(1), h.rank());
    }

    #[test]
    fn test_rank_seven_four_kind() {
        let h = Hand::new_from_strs(&["s2", "h2", "d2", "c2", "dk", "h9", "s4"]).unwrap();
        let four_rank = (1 << card::Rank::Two as u16) << 13;
        let low_rank = 1 << card::Rank::King as u16;
        assert_eq!(Rank::FourOfAKind(four_rank | low_rank), h.rank());
    }

    #[test]
    fn test_rank_seven_four_plus_set() {
        // Four of a kind plus a set.
        let h = Hand::new_from_strs(&["s2", "h2", "d2", "c2", "d8", "s8", "c8"]).unwrap();
        let four_rank = (1 << card::Rank::Two as u16) << 13;
        let low_rank = 1 << card::Rank::Eight as u16;
        assert_eq!(Rank::FourOfAKind(four_rank | low_rank), h.rank());
    }

    #[test]
    fn test_rank_seven_full_house_two_sets() {
        // We have two sets use the highest set.
        let h = Hand::new_from_strs(&["sa", "h2", "d2", "c2", "d8", "s8", "c8"]).unwrap();
        let set_rank = (1 << card::Rank::Eight as u16) << 13;
        let low_rank = 1 << card::Rank::Two as u16;
        assert_eq!(Rank::FullHouse(set_rank | low_rank), h.rank());
    }

    #[test]
    fn test_rank_seven_full_house_two_pair() {
        // Test to make sure that we pick the best pair.
        let h = Hand::new_from_strs(&["h2", "d2", "c2", "d8", "s8", "dk", "sk"]).unwrap();
        let set_rank = (1 << card::Rank::Two as u16) << 13;
        let low_rank = 1 << card::Rank::King as u16;
        assert_eq!(Rank::FullHouse(set_rank | low_rank), h.rank());
    }

    #[test]
    fn test_two_pair_from_three_pair() {
        let h = Hand::new_from_strs(&["h2", "d2", "d8", "s8", "dk", "sk", "ht"]).unwrap();
        let pair_rank = ((1 << card::Rank::King as u16) | (1 << card::Rank::Eight as u16)) << 13;
        let low_rank = 1 << card::Rank::Ten as u16;
        assert_eq!(Rank::TwoPair(pair_rank | low_rank), h.rank());
    }

    #[test]
    fn test_rank_seven_two_pair() {
        let h = Hand::new_from_strs(&["h2", "d2", "d8", "s8", "dk", "s6", "ht"]).unwrap();
        let pair_rank = ((1 << card::Rank::Two as u16) | (1 << card::Rank::Eight as u16)) << 13;
        let low_rank = 1 << card::Rank::King as u16;
        assert_eq!(Rank::TwoPair(pair_rank | low_rank), h.rank());
    }

    #[test]
    fn test_compare_ranks() {
        let ranks = vec![
            Rank::HighCard(1),
            Rank::OnePair(1 << 13),
            Rank::OnePair(1 << 13),
            Rank::TwoPair(2 << 13),
        ];
        assert_eq!(compare_ranks(&ranks), vec![3]);

        let ranks = vec![
            Rank::HighCard(1),
            Rank::OnePair(1 << 13),
            Rank::TwoPair(2 << 13),
            Rank::TwoPair(2 << 13),
        ];
        assert_eq!(compare_ranks(&ranks), vec![2, 3]);

        let ranks = vec![Rank::HighCard(1)];
        assert_eq!(compare_ranks(&ranks), vec![0]);

        let ranks: Vec<Rank> = vec![];
        assert_eq!(compare_ranks(&ranks), vec![]);
    }
}
