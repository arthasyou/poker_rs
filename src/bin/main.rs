use poker_rs::poker::{card::Card, hand::Hand};

fn main() {
    let hand_str = vec!["SA", "ht", "D9", "c2"];
    let mut hand = Hand::new_from_strs(&hand_str).unwrap();
    println!("hand: {}", hand);

    // let card = Card::try_from_str("ct").unwrap();

    // hand.push(card);

    hand.remove(1);

    println!("hand: {}", hand);
}
