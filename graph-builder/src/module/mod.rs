use crate::api::{bulk_fetch_module_infos, fetch_all_modules};
use crate::errors::ModuleError;
use crate::models::{Module, ModuleInfo};
use crate::storage::{self, Storable};

pub async fn get_all_module_infos() -> Result<Vec<ModuleInfo>, ModuleError> {
    if let Some(module_infos) = storage::read_collection::<ModuleInfo>() {
        println!(
            "Loaded {} module infos from storage at {}",
            module_infos.len(),
            ModuleInfo::path()
        );
        Ok(module_infos)
    } else {
        let modules = get_all_modules().await?;
        let module_codes = modules
            .iter()
            .map(|module| module.module_code.as_str())
            .collect::<Vec<_>>();

        let responses = bulk_fetch_module_infos(&module_codes).await?;
        let deserialised: Vec<ModuleInfo> = responses
            .into_iter()
            .map(|result| serde_json::from_str::<ModuleInfo>(&result))
            .collect::<Result<_, _>>()?;

        storage::write_collection::<ModuleInfo>(&deserialised)?;
        Ok(deserialised)
    }
}

async fn get_all_modules() -> Result<Vec<Module>, ModuleError> {
    if let Some(modules) = storage::read_collection::<Module>() {
        println!(
            "Loaded {} modules from storage at {}",
            modules.len(),
            Module::path()
        );
        Ok(modules)
    } else {
        let modules = fetch_all_modules().await?;
        let modules: Vec<Module> = serde_json::from_str(&modules)?;
        storage::write_collection::<Module>(&modules)?;
        Ok(modules)
    }
}
