use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let result = aoc::customs_declaration::groups_from_str(&data)
        .iter()
        .fold(0, |v, p| v + p.answer_count());

    info!("Day 06a: {}", result);
}
