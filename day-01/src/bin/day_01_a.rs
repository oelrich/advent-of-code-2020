use tracing::{error, info};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(default_value = "info")]
    filter: String,
}

fn main() {
    let cli = Cli::from_args();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(cli.filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut reader = csv::Reader::from_path(cli.input_path).expect("input missing");
    let mut values = Vec::default();
    for result in reader.deserialize() {
        let value: i32 = result.expect("could not parse result");
        values.push(value);
    }
    match day_01::find_sum(2020, values) {
        Some((a, b)) => info!("Day 01a: {}", a * b),
        None => error!("Could not find pair"),
    };
}
