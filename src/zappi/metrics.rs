use std::convert::Infallible;
use lazy_static::lazy_static;
use prometheus::{Gauge, register_gauge, TextEncoder, Encoder};
use hyper::{header::CONTENT_TYPE, Body, Request, Response};
use log::info;

use super::model::ApiResponse;

lazy_static! {

    pub static ref POWER_FREQUENCY: Gauge = register_gauge!(
        "power_freq",
        "The hz of power frequency",
    )
    .unwrap();

    pub static ref SUPPLY_VOLTAGE: Gauge = register_gauge!(
        "supply_voltage",
        "Current supply voltage",
    )
    .unwrap();

    pub static ref GRID_USAGE: Gauge = register_gauge!(
        "grid_usage",
        "Current grid ussage in watts",
    )
    .unwrap();

}

pub fn set_stats(zappi_response: Option<ApiResponse>) {

    if let Some(zappi) = zappi_response {

        POWER_FREQUENCY.set(zappi.power_freq);

        info!(
            "-> setting power frequency (hz): {}",
            zappi.power_freq
        );

        SUPPLY_VOLTAGE.set(zappi.supply_voltage);

        info!(
            "-> setting supply voltage (v): {}",
            zappi.supply_voltage
        );

        GRID_USAGE.set(zappi.grid_usage);

        info!(
            "-> setting grid usuage (w): {}",
            zappi.grid_usage
        );
    }
}

pub async fn renderer(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let metrics = prometheus::gather();
    let mut buffer = vec![];

    let encoder = TextEncoder::new();
    encoder.encode(&metrics, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}