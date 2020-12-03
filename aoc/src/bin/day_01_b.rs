use tracing::{error, info};

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let mut reader = csv::Reader::from_path(input_path).expect("input missing");
    let mut values = Vec::default();
    for result in reader.deserialize() {
        let value: i32 = result.expect("could not parse result");
        values.push(value);
    }
    match aoc::sums::find_triplet_sum(2020, values) {
        Some((a, b, c)) => info!("Day 01b: {}", a * b * c),
        None => error!("Could not find triplet"),
    };
}
