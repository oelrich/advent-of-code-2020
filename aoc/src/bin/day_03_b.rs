use std::str::FromStr;

use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let map = aoc::trees::Map::from_str(&data).expect("a map");
    let mut result = 1;
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .cloned()
        .for_each(|(x, y)| result *= map.encounters(x, y));

    info!("Day 03b: {}", result);
}
