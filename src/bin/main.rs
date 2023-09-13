use clap::Parser;
use rs_template::init_telemetry;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct App {}

#[tokio::main]
async fn main() {
    init_telemetry();

    let app = App::parse();

    tracing::info!("hello world");
    tracing::warn!("OH NO");
    tracing::debug!("{:?}", app);
    tracing::trace!("{:#?}", app);
}
