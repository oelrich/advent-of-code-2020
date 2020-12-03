use std::str::FromStr;

use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let map = aoc::trees::Map::from_str(&data).expect("a map");
    info!("Day 03a: {}", map.encounters(3, 1));
}
