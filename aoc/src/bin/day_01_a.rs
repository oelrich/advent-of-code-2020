use tracing::{error, info};

fn main() {
    let input_path = aoc::setup::init_and_get_input();
    let mut reader = csv::Reader::from_path(input_path).expect("input missing");
    let mut values = Vec::default();
    for result in reader.deserialize() {
        let value: i32 = result.expect("could not parse result");
        values.push(value);
    }
    match aoc::sums::find_sum(2020, values) {
        Some((a, b)) => info!("Day 01a: {}", a * b),
        None => error!("Could not find pair"),
    };
}
