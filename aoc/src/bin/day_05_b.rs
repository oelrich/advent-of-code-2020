use tracing::info;

fn main() {
    let input_path = aoc::setup::init_and_get_input();

    let data = std::fs::read_to_string(input_path).expect("data");
    let seat_ids = aoc::boarding_pass::boarding_passes_from_string(&data)
        .iter()
        .map(|p| {
            let (_, _, sid) = p.seat();
            sid
        })
        .collect::<std::collections::HashSet<i32>>();
    //    let max = seat_ids.iter().cloned().max().unwrap();
    let mut result = 0;
    loop {
        if !seat_ids.contains(&result) {
            let left = result + 1;
            let right = result - 1;
            if seat_ids.contains(&left) && seat_ids.contains(&right) {
                break;
            }
        }
        result += 1;
    }
    info!("Day 05b: {}", result);
}
