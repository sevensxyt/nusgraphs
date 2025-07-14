#[derive(Debug)]
pub enum GraphBuilderError {
    Module,
}

impl From<ModuleError> for GraphBuilderError {
    fn from(_error: ModuleError) -> Self {
        Self::Module
    }
}

#[derive(Debug)]
pub enum ModuleError {
    Api,
    Serde,
    Reqwest,
    Storage,
}

impl From<ApiError> for ModuleError {
    fn from(_error: ApiError) -> Self {
        Self::Api
    }
}

impl From<serde_json::Error> for ModuleError {
    fn from(_error: serde_json::Error) -> Self {
        Self::Serde
    }
}

impl From<reqwest::Error> for ModuleError {
    fn from(_error: reqwest::Error) -> Self {
        Self::Reqwest
    }
}

impl From<StorageError> for ModuleError {
    fn from(_error: StorageError) -> Self {
        Self::Storage
    }
}

pub enum ApiError {
    Reqwest,
}

impl From<reqwest::Error> for ApiError {
    fn from(_error: reqwest::Error) -> Self {
        Self::Reqwest
    }
}

pub enum StorageError {
    Io,
    Serde,
}

impl From<std::io::Error> for StorageError {
    fn from(_error: std::io::Error) -> Self {
        Self::Io
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(_error: serde_json::Error) -> Self {
        Self::Serde
    }
}
