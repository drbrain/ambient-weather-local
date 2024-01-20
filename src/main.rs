mod ambient_weather_report;
mod descriptor;
mod gauge;
mod info;
mod metrics;
mod report;
mod reports;
mod server;

pub use crate::{
    descriptor::Descriptor, gauge::Gauge, info::Info, metrics::Metrics, report::Report,
    reports::Reports,
};
use env_logger::Env;
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let address = "0.0.0.0:8080".parse().unwrap();

    Server::new(address).start().await?;

    Ok(())
}
