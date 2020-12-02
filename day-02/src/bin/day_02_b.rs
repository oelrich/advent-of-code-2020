use tracing::info;
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
    info!("Day 02b: {}", day_02::count_valid_1(&cli.input_path));
}
