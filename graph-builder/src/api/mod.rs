use crate::errors::ApiError;
use futures::stream::{self, StreamExt};

const CONCURRENCY: usize = 10;

pub const API_URL: &str = "https://api.nusmods.com/v2/2025-2026";

pub async fn fetch_all_modules() -> Result<String, ApiError> {
    let url = format!("{API_URL}/moduleList.json");
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn bulk_fetch_module_infos(module_codes: &[&str]) -> Result<Vec<String>, ApiError> {
    let responses = stream::iter(module_codes)
        .map(|module_code| fetch_module_info(module_code))
        .buffer_unordered(CONCURRENCY)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(responses)
}

async fn fetch_module_info(module_code: &str) -> Result<String, ApiError> {
    let url = format!("{API_URL}/modules/{module_code}.json");
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
