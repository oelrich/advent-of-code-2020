use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let mut machine = aoc::intligen::load_machine(&data);

    info!("Day 08a: {}", machine.run_to_first());
}
