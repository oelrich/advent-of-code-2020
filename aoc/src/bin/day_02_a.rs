use tracing::info;
fn main() {
    let input_path = aoc::setup::init_and_get_input();
    info!("Day 02a: {}", aoc::passwords::count_valid(&input_path));
}
