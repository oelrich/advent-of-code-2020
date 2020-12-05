use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let result = aoc::boarding_pass::boarding_passes_from_string(&data)
        .iter()
        .map(|p| {
            let (_, _, sid) = p.seat();
            sid
        })
        .max()
        .unwrap();

    info!("Day 05b: {}", result);
}
