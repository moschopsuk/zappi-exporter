mod zappi;

use env_logger::{Builder as LoggerBuilder, Env};
use log::{error, info};
use config::Config;
use hyper::{service::make_service_fn, service::service_fn, Server};
use std::convert::Infallible;
use std::time::Duration;
use ticker::Ticker;

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

        let ticker = Ticker::new((0..).cycle(), Duration::from_secs(ticker));
        for _ in ticker {
            metrics::set_stats(zappi_client.retrieve_stats().await);
        }
    });
}



// use config::Config;
// use actix_web::{get, App, HttpServer, Responder};
// use actix_web_prom::PrometheusMetricsBuilder;
// use prometheus::Gauge;

// use reqwest::blocking::Client;
// use reqwest::header::{CONTENT_TYPE, ACCEPT};
// use diqwest::blocking::WithDigestAuth;

// use std::{thread, panic};
// use std::time::Duration;
// use std::process;

// use simple_logger::SimpleLogger;
// use log::{LevelFilter, info};

// #[get("/")]
// async fn index() -> impl Responder {
//     "Hello, World!"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

//     let settings = Config::builder()
//         .add_source(config::File::with_name("config"))
//         .add_source(config::Environment::with_prefix("APP"))
//         .build()
//         .unwrap();

//     let prometheus = PrometheusMetricsBuilder::new("api")
//         .endpoint("/metrics")
//         .build()
//         .unwrap();

//     let power_freq = Gauge::new("power_freq", "Current power frequency").unwrap();
//     let supply_voltage = Gauge::new("supply_voltage", "Current supply voltage").unwrap();
//     let grid_usage = Gauge::new("grid_usage", "Current grid ussage").unwrap();

//     prometheus
//         .registry
//         .register(Box::new(power_freq.clone()))
//         .unwrap();

//     prometheus
//         .registry
//         .register(Box::new(supply_voltage.clone()))
//         .unwrap();

//     prometheus
//         .registry
//         .register(Box::new(grid_usage.clone()))
//         .unwrap();

//     let orig_hook = panic::take_hook();
//     panic::set_hook(Box::new(move |panic_info| {
//         // invoke the default handler and exit the process
//         orig_hook(panic_info);
//         process::exit(1);
//     }));

//     thread::spawn(move || loop {
//         let client = Client::new();
//         let url: String = settings.get_string("endpoint").unwrap();
//         let serial: String = settings.get_string("serial").unwrap();
//         let apikey: String = settings.get_string("apikey").unwrap();

//         let response = client.get::<String>(url)
//             .header(CONTENT_TYPE, "application/json")
//             .header(ACCEPT, "application/json")
//             .send_with_digest_auth(serial.as_str(), apikey.as_str())
//             .unwrap()
//             .json::<serde_json::Value>()
//             .unwrap();

//         let zappi = response["zappi"].as_array().unwrap().get(0).unwrap();

//         power_freq.set(zappi["frq"].as_f64().unwrap());
//         //TODO: This is broken in the current zappi API and needs to be devided by 10
//         supply_voltage.set(zappi["vol"].as_f64().unwrap() / 10.0); 
//         grid_usage.set(zappi["grd"].as_f64().unwrap());

//         thread::sleep(Duration::from_secs(5));
//     });

//     info!("starting server 0.0.0.0:4000");

//     HttpServer::new(move || {
//         App::new()
//             .wrap(prometheus.clone())
//             .service(index)
//     })
//     .bind(("0.0.0.0", 4000))?
//     .run()
//     .await
// }
