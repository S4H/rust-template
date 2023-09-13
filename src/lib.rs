use tracing_subscriber::{filter::LevelFilter, EnvFilter};

pub fn init_telemetry() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(filter)
        .with_file(true)
        .with_line_number(true)
        .init();
}
