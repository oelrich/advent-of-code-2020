use structopt::StructOpt;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};
#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(default_value = "info")]
    filter: String,
}

pub fn init_and_get_input() -> std::path::PathBuf {
    let cli = Cli::from_args();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(cli.filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    cli.input_path
}
