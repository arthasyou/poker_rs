use poker_rs::holdem::evaluator::range::calculate_range_percent;

fn main() {
    match calculate_range_percent("88+, 22+") {
        Ok(percent) => println!("Range percent: {:.2}%", percent * 100.0),
        Err(err) => println!("Error: {:?}", err),
    }
}
