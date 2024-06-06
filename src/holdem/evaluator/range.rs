//! # Poker Hand Range Percentage Calculator
//!
//! This module provides a function to calculate the percentage of poker hand combinations
//! represented by a given hand range string.
//!
//! ## Overview
//!
//! In Texas Hold'em poker, hand ranges can be specified using exact hands (e.g., `AKo`, `AA`, `KTs`)
//! or ranges (e.g., `AKo+`, `22+`). This module provides the `calculate_range_percent` function
//! to compute the percentage of all possible hand combinations represented by a given hand range string.
//!
//! ## Important Note
//!
//! When using this function, ensure that the hand ranges do not contain overlapping or conflicting parts.
//! For example, avoid specifying `88+, 22+` or `KT+, K9s+` in the same input string, as these will result
//! in double counting of combinations.The function includes a mechanism to detect and remove such duplicates,
//! but it is best practice to input non-overlapping ranges to ensure accurate calculation.
//!
//! //! ## Example
//!
//! ```rust
//! use poker_rs::holdem::evaluator::range::calculate_range_percent;
//!
//! fn main() {
//!     match calculate_range_percent("88+, AJo+, ATs+") {
//!         Ok(percent) => println!("Range percent: {:.2}%", percent * 100.0),
//!         Err(err) => println!("Error: {:?}", err),
//!     }
//! }
//! ```

use std::collections::HashSet;

use crate::{
    error::{Error, Result},
    poker::card::Rank,
};
use once_cell::sync::Lazy;
use regex::Regex;

use super::hand_type::HandType;

const HAND_COMBINATIONS: f32 = 1326.0;

// const OFF_SUIT_COMBINATIONS: u16 = 936;
// const SUITED_COMBINATIONS: u16 = 312;
// const PAIRED_COMBINATIONS: u16 = 78;

const SPEC_OFF_SUIT_COMBINATIONS: usize = 12;
const SPEC_SUITED_COMBINATIONS: usize = 4;
const SPEC_PAIRED_COMBINATIONS: usize = 6;
// const SPEC_UNPAIRED_COMBINATIONS: usize = 16;

// const PAIRED_COUNT: u16 = 13;
// const UNPAIRED_COUNT: u16 = 78;

const RANGE_PAT: &str = r"(?i)^(?:[AKQJTt2-9]{2}[os]?\+?)$";
static RANGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(RANGE_PAT).unwrap());
static TRIM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s*([,])\s*").unwrap());

struct Combinations {
    offsuit: HashSet<String>,
    suited: HashSet<String>,
    paired: HashSet<String>,
}

impl Combinations {
    pub fn new() -> Self {
        Self {
            offsuit: HashSet::new(),
            suited: HashSet::new(),
            paired: HashSet::new(),
        }
    }

    pub fn len_of_offsuit(&self) -> usize {
        self.offsuit.len()
    }

    pub fn len_of_suited(&self) -> usize {
        self.suited.len()
    }

    pub fn len_of_paired(&self) -> usize {
        self.paired.len()
    }
}

/// Calculates the percentage of hand combinations represented by the input string.
///
/// # Arguments
///
/// * `hand_range` - A string slice that holds the hand range.
///
/// # Returns
///
/// * `Result<f32, Error>` - The percentage of hand combinations.
///
/// # Errors
///
/// * `Error::UnexpectedCardChar` - If the input string contains unexpected characters.
// pub fn calculate_range_percent(s: &str) -> Result<f32> {
//     let s = TRIM_REGEX.replace_all(s, "$1").trim().to_string();
//     let ranges: Vec<&str> = s.split(',').collect::<Vec<_>>();
//     let mut total_combinations = 0_u16;
//     for range in ranges.into_iter() {
//         let caps = RANGE_REGEX
//             .captures(range)
//             .ok_or_else(|| Error::UnexpectedCardChar)?;

//         let matched_range = &caps[0];

//         let combination_count = if range.contains('+') {
//             calculate_plus_range(matched_range)
//         } else {
//             calculate_single_range(matched_range)
//         };
//         total_combinations += combination_count;
//     }
//     Ok(total_combinations as f32 / HAND_COMBINATIONS)
// }

pub fn calculate_range_percent(s: &str) -> Result<f32> {
    let s = TRIM_REGEX.replace_all(s, "$1").trim().to_string();
    let ranges: Vec<&str> = s.split(',').collect::<Vec<_>>();
    let mut combinations = Combinations::new();
    for range in ranges.into_iter() {
        let caps = RANGE_REGEX
            .captures(range)
            .ok_or_else(|| Error::UnexpectedCardChar)?;

        let matched_range = &caps[0];

        if range.contains('+') {
            generate_plus_combinations(matched_range, &mut combinations)
        } else {
            generate_single_combinations(matched_range, &mut combinations)
        };
    }

    let total_count = combinations.len_of_offsuit() * SPEC_OFF_SUIT_COMBINATIONS
        + combinations.len_of_suited() * SPEC_SUITED_COMBINATIONS
        + combinations.len_of_paired() * SPEC_PAIRED_COMBINATIONS;
    Ok(total_count as f32 / HAND_COMBINATIONS)
}

fn generate_plus_combinations(s: &str, combinations: &mut Combinations) {
    let (rank1, rank2, hand_type) = parse_cards(s);

    match hand_type {
        HandType::Offsuit => {
            let gap = rank1.gap(&rank2);
            for i in 0..gap {
                let v = rank2.as_int() + i;
                combinations
                    .offsuit
                    .insert(format!("{}{}", rank1, Rank::from_int(v).unwrap()));
            }
        }
        HandType::Suited => {
            let gap = rank1.gap(&rank2);
            for i in 0..gap {
                let v = rank2.as_int() + i;
                combinations
                    .suited
                    .insert(format!("{}{}", rank1, Rank::from_int(v).unwrap()));
            }
        }
        HandType::Paired => {
            let gap = rank1.gap_with_ace();
            for i in 0..=gap {
                let v = rank1.as_int() + i;
                combinations
                    .paired
                    .insert(format!("{}", Rank::from_int(v).unwrap()));
            }
        }
        HandType::UnPaired => {
            let gap = rank1.gap(&rank2);
            for i in 0..gap {
                let v = rank2.as_int() + i;
                combinations
                    .offsuit
                    .insert(format!("{}{}", rank1, Rank::from_int(v).unwrap()));
                combinations
                    .suited
                    .insert(format!("{}{}", rank1, Rank::from_int(v).unwrap()));
            }
        }
    }
}

fn generate_single_combinations(s: &str, combinations: &mut Combinations) {
    let (rank1, rank2, hand_type) = parse_cards(s);
    match hand_type {
        HandType::Offsuit => {
            combinations.offsuit.insert(format!("{}{}", rank1, rank2));
        }
        HandType::Suited => {
            combinations.suited.insert(format!("{}{}", rank1, rank2));
        }
        HandType::Paired => {
            combinations.paired.insert(format!("{}", rank1));
        }
        HandType::UnPaired => {
            combinations.offsuit.insert(format!("{}{}", rank1, rank2));
            combinations.suited.insert(format!("{}{}", rank1, rank2));
        }
    }
}

// fn calculate_plus_range(s: &str) -> usize {
//     let (rank1, rank2, hand_type) = parse_cards(s);

//     match hand_type {
//         HandType::Offsuit => {
//             let count = rank1.gap(&rank2) as usize;
//             SPEC_OFF_SUIT_COMBINATIONS * count
//         }
//         HandType::Suited => {
//             let count = rank1.gap(&rank2) as usize;
//             SPEC_SUITED_COMBINATIONS * count
//         }
//         HandType::Paired => {
//             let count = (rank1.gap_with_ace() + 1) as usize;
//             SPEC_PAIRED_COMBINATIONS * count
//         }
//         HandType::UnPaired => {
//             let count = rank1.gap(&rank2) as usize;
//             SPEC_UNPAIRED_COMBINATIONS * count
//         }
//     }
// }

// fn calculate_single_range(s: &str) -> u16 {
//     let (_, _, hand_type) = parse_cards(s);
//     match hand_type {
//         HandType::Offsuit => SPEC_OFF_SUIT_COMBINATIONS,
//         HandType::Suited => SPEC_SUITED_COMBINATIONS,
//         HandType::Paired => SPEC_PAIRED_COMBINATIONS,
//         HandType::UnPaired => SPEC_UNPAIRED_COMBINATIONS,
//     }
// }

fn parse_cards(s: &str) -> (Rank, Rank, HandType) {
    let mut chars = s.chars();
    let rank1 = Rank::from_char(chars.next().unwrap()).unwrap();
    let rank2 = Rank::from_char(chars.next().unwrap()).unwrap();
    let hand_type = match chars.next() {
        Some('o') => HandType::Offsuit,
        Some('s') => HandType::Suited,
        _ => {
            if rank1 == rank2 {
                HandType::Paired
            } else {
                HandType::UnPaired
            }
        }
    };

    (rank1, rank2, hand_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_combinations() {
        let valid_combinations = vec![
            "AKo", "AAs", "23", "TT", "QJo", "QJs", "97o", "86s", "AKo+", "q2+",
        ];

        for combo in valid_combinations {
            assert!(
                RANGE_REGEX.is_match(combo),
                "Expected {} to be a valid combo",
                combo
            );
        }
    }

    #[test]
    fn test_invalid_combinations() {
        let invalid_combinations = vec![
            "AKx", "AAos", "11", "ZZ", "A", "K", "AK+QJ", "AKo++", "AAs--", "-Aks", "AKo-A2o",
        ];

        for combo in invalid_combinations {
            assert!(
                !RANGE_REGEX.is_match(combo),
                "Expected {} to be an invalid combo",
                combo
            );
        }
    }

    #[test]
    fn test_calculate_range_percent() {
        let test_cases = [
            ("KK+", 0.009_f32),
            ("JJ+, AK", 0.0302_f32),
            ("99+, AQ+", 0.0513_f32),
            ("88+, AJo+, ATs+", 0.0709_f32),
            ("77+, ATo+, A8s+, KQ, KQo, KQs", 0.1026_f32),
            ("66+, A8o+, A5s+, KJo+, KTs+, QJs", 0.1523_f32),
            (
                "22+, A2+, K4o+, K2s+, Q6o+, Q3s+, J8o+, J7s+, T9o+, T7s+, 98o+, 97s+, 87o+, 86s+, 75s+, 65s, 54s",
                0.4992_f32,
            ),
            ("22+, 33+", 0.05882_f32),
        ];

        for (input, expected) in test_cases {
            let actual = calculate_range_percent(input).unwrap();
            let diff = (actual - expected).abs();
            assert!(
                diff < 0.0001,
                "Expected {:.5}, but got {:.5}",
                expected,
                actual
            );
        }
    }

    #[test]
    fn test_calculate_range_percent_invalid_input() {
        let invalid_inputs = [
            "AKx", "AAos", "11", "ZZ", "A", "K", "AK+QJ", "AKo++", "AAs--", "-Aks",
        ];

        for input in invalid_inputs.iter() {
            let result = calculate_range_percent(input);
            assert!(result.is_err(), "Expected error for input: {}", input);
        }
    }
}
