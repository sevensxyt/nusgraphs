use crate::errors::ApiError;

pub const API_URL: &str = "https://api.nusmods.com/v2/2025-2026";

pub async fn fetch_all_modules() -> Result<String, ApiError> {
    let url = format!("{API_URL}/moduleList.json");
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn fetch_module_info(module_code: &str) -> Result<String, ApiError> {
    let url = format!("{API_URL}/modules/{module_code}.json");
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
