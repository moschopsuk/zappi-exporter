use config::Config;
use actix_web::{get, App, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Gauge;

use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use diqwest::blocking::WithDigestAuth;

use std::thread;
use std::time::Duration;

use simple_logger::SimpleLogger;
use log::{LevelFilter, info};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    let power_freq = Gauge::new("power_freq", "Current power frequency").unwrap();
    let grid_usage = Gauge::new("grid_usage", "Current grid ussage").unwrap();

    prometheus
        .registry
        .register(Box::new(power_freq.clone()))
        .unwrap();

    prometheus
        .registry
        .register(Box::new(grid_usage.clone()))
        .unwrap();

    thread::spawn(move || loop {
        let client = Client::new();
        let url: String = settings.get_string("endpoint").unwrap();
        let serial: String = settings.get_string("serial").unwrap();
        let apikey: String = settings.get_string("apikey").unwrap();
         
        let response = client.get::<String>(url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send_with_digest_auth(serial.as_str(), apikey.as_str())
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();

        let zappi = response["zappi"].as_array().unwrap().get(0).unwrap();

        power_freq.set(zappi["frq"].as_f64().unwrap());
        grid_usage.set(zappi["grd"].as_f64().unwrap());

        thread::sleep(Duration::from_secs(5));
    });

    info!("starting server 127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
