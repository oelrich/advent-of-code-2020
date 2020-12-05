use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let passports = aoc::passports::Passport::passports_from_str(&data);
    info!("Day 04b: 164 too high {}", passports.len());
}
