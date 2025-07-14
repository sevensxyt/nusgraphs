use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphBuilderError {
    #[error("module error: {0}")]
    Module(#[from] ModuleError),
}

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("api error: {0}")]
    Api(#[from] ApiError),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
