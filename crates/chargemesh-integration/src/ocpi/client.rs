//! OCPI Client (for EMSP side)

use super::*;
use reqwest::{Client, StatusCode};
use std::time::Duration;

pub struct OcpiClient {
    client: Client,
    base_url: String,
    token: String,
    version: OcpiVersion,
    country_code: String,
    party_id: String,
}

impl OcpiClient {
    pub fn new(
        base_url: &str,
        token: &str,
        version: OcpiVersion,
        country_code: &str,
        party_id: &str,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            base_url: base_url.trim_end_matches('/').to_string(),
            token: token.to_string(),
            version,
            country_code: country_code.to_string(),
            party_id: party_id.to_string(),
        }
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let url = format!("{}/ocpi/{}/{}/{}",
            self.base_url,
            self.version,
            self.country_code,
            self.party_id,
            path.trim_start_matches('/')
        );

        let mut request = self.client.request(method, &url);
        request = request.header("Authorization", format!("Token {}", self.token));
        request = request.header("OCPI-From-Country-Code", &self.country_code);
        request = request.header("OCPI-From-Party-Id", &self.party_id);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await
            .map_err(|e| IntegrationError::Ocpi(e.to_string()))?;

        if !response.status().is_success() {
            return Err(IntegrationError::Ocpi(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let data = response.json::<T>().await
            .map_err(|e| IntegrationError::Ocpi(e.to_string()))?;

        Ok(data)
    }

    pub async fn get_locations(&self) -> Result<Vec<OcpiLocation>> {
        #[derive(serde::Deserialize)]
        struct Response {
            data: Vec<OcpiLocation>,
        }
        let response: Response = self.request(reqwest::Method::GET, "locations", None).await?;
        Ok(response.data)
    }

    pub async fn get_sessions(&self) -> Result<Vec<OcpiSession>> {
        #[derive(serde::Deserialize)]
        struct Response {
            data: Vec<OcpiSession>,
        }
        let response: Response = self.request(reqwest::Method::GET, "sessions", None).await?;
        Ok(response.data)
    }

    pub async fn create_cdr(&self, cdr: &OcpiCdr) -> Result<OcpiCdr> {
        #[derive(serde::Deserialize)]
        struct Response {
            data: OcpiCdr,
        }
        let response: Response = self.request(
            reqwest::Method::POST,
            "cdrs",
            Some(serde_json::to_value(cdr).unwrap()),
        ).await?;
        Ok(response.data)
    }
}