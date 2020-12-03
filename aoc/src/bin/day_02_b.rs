use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();
    info!("Day 02b: {}", aoc::passwords::count_valid_1(&input_path));
}
