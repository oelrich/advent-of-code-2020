use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let result = aoc::bagateller::shiny_count(&data);

    info!("Day 07b: {}", result);
}
