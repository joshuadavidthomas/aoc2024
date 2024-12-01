use anyhow::Result;
use reqwest::StatusCode;

use crate::AOC_BASE_URL;

pub struct AocClient {
    client: reqwest::Client,
    session: String,
}

impl AocClient {
    pub fn new(session: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            session,
        }
    }

    pub async fn get_problem(&self, day: u32) -> Result<String> {
        let url = format!("{}{}", AOC_BASE_URL, day);
        let response = self
            .client
            .get(&url)
            .header("Cookie", format!("session={}", self.session))
            .send()
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => anyhow::bail!("Day {} is not available yet", day),
            StatusCode::OK => Ok(response.text().await?),
            status => anyhow::bail!("Unexpected status code: {}", status),
        }
    }

    pub async fn get_input(&self, day: u32) -> Result<String> {
        let url = format!("{}{}/input", AOC_BASE_URL, day);
        let response = self
            .client
            .get(&url)
            .header("Cookie", format!("session={}", self.session))
            .send()
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => anyhow::bail!("Input for day {} is not available yet", day),
            StatusCode::OK => Ok(response.text().await?),
            status => anyhow::bail!("Unexpected status code: {}", status),
        }
    }
}
