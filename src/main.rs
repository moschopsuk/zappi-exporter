mod zappi;

use env_logger::{Builder as LoggerBuilder, Env};
use log::{error, info};
use config::Config;
use hyper::{service::make_service_fn, service::service_fn, Server};
use std::convert::Infallible;
use std::time::Duration;

use zappi::client::Client as ZappiClient;
use zappi::metrics;

#[tokio::main]
async fn main() {
    LoggerBuilder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

    // start ticker
    run_ticker(config);

    // set up http server
    let addr = ([0, 0, 0, 0], 9888).into();
    info!("starting zappi exporter on address: {:?}", addr);

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(metrics::renderer)) });

    let server = Server::bind(&addr).serve(make_svc);

    // start HTTP server
    if let Err(e) = server.await {
        error!("a server error occurred: {}", e);
    }
}

fn run_ticker(config: Config) {
    tokio::spawn(async move {
        let url: String = config.get_string("endpoint").unwrap();
        let serial: String = config.get_string("serial").unwrap();
        let api_key: String = config.get_string("apikey").unwrap();
        let ticker: u64 = config.get_int("ticker").unwrap().try_into().unwrap();

        let mut zappi_client = ZappiClient::new(url, serial, api_key);

        info!("waiting for the first tick in {} seconds...", ticker);

        loop {
            tokio::time::sleep(Duration::from_secs(ticker)).await;
            metrics::set_stats(zappi_client.retrieve_stats().await);
        }
    });
}
