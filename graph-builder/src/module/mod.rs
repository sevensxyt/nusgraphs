use crate::api::{fetch_all_modules, fetch_module_info};
use crate::errors::ModuleError;
use crate::models::{Module, ModuleInfo};
use crate::storage;
use futures::future::try_join_all;

const CHUNK_SIZE: usize = 100;

pub async fn get_all_module_infos() -> Result<Vec<ModuleInfo>, ModuleError> {
    if let Some(module_infos) = storage::read_module_infos() {
        println!("Loaded {} module infos from storage", module_infos.len());
        Ok(module_infos)
    } else {
        let modules = get_all_modules().await?;
        let mut module_infos = Vec::new();

        for chunk in modules.chunks(CHUNK_SIZE) {
            let futures = chunk
                .iter()
                .map(|module| fetch_module_info(&module.module_code));
            let results = try_join_all(futures).await?;

            let deserialised: Vec<ModuleInfo> = results
                .into_iter()
                .map(|result| serde_json::from_str::<ModuleInfo>(&result))
                .collect::<Result<_, _>>()?;
            module_infos.extend(deserialised);

            println!("Processed {} module infos", module_infos.len());
        }

        storage::write_module_infos(&module_infos)?;
        Ok(module_infos)
    }
}

async fn get_all_modules() -> Result<Vec<Module>, ModuleError> {
    Ok(match storage::read_modules() {
        Some(modules) => {
            println!("Loaded {} modules from storage", modules.len());
            modules
        }
        None => {
            let modules = fetch_all_modules().await?;
            let modules: Vec<Module> = serde_json::from_str(&modules)?;
            storage::write_modules(&modules)?;
            modules
        }
    })
}
