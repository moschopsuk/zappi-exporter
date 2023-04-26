use reqwest::header::{CONTENT_TYPE, ACCEPT};
use diqwest::WithDigestAuth;
use log::{error, info};

use super::model::ApiResponse;
pub struct Client {
    http_client: reqwest::Client,
    base_url: reqwest::Url,
    serial: String,
    api_key: String,
}

impl Client {
    pub fn new(base_url: String, serial: String, api_key: String) -> Client {
        Client {
            http_client: reqwest::Client::new(),
            base_url: base_url.parse().unwrap(),
            serial,
            api_key,
        }
    }

    async fn get(&self) -> Result<reqwest::Response, anyhow::Error> {
        self.http_client
            .get(self.base_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send_with_digest_auth(self.serial.as_str(), self.api_key.as_str())
            .await
            .map_err(anyhow::Error::from)
    }

    async fn stats(&self) -> Result<serde_json::Value, anyhow::Error> {
        let resp = self.get().await?;
        resp.json::<serde_json::Value>().await.map_err(anyhow::Error::from)
    }

    pub async fn retrieve_stats(&mut self) -> Option<ApiResponse> {
        info!("retrieving zappi stats ...");

        let zappi_response = match self.stats().await {
            Ok(resp) => {
                let zappi = resp["zappi"].as_array().unwrap().get(0).unwrap();
                
                ApiResponse {
                    power_freq: zappi["frq"].as_f64().unwrap(),
                    supply_voltage: zappi["vol"].as_f64().unwrap() / 10.0,
                    grid_usage: zappi["grd"].as_f64().unwrap(),
                }
            },
            Err(e) => {
                error!("unable to retrieve zappi stats: {}", e);
                return None;
            }
        };

        Some(zappi_response)
    }
}