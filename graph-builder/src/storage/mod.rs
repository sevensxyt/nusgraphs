use crate::errors::StorageError;
use crate::models::{Module, ModuleInfo};
use std::fs::File;
use std::io::{BufReader, BufWriter};

const MODULE_CODES_PATH: &str = "data/modules.json";
const MODULE_INFO_PATH: &str = "data/module_infos.json";

pub fn read_modules() -> Option<Vec<Module>> {
    let file = File::open(MODULE_CODES_PATH).ok()?;
    let reader = BufReader::new(file);
    let modules: Vec<Module> = serde_json::from_reader(reader).ok()?;
    Some(modules)
}

pub fn read_module_infos() -> Option<Vec<ModuleInfo>> {
    let file = File::open(MODULE_INFO_PATH).ok()?;
    let reader = BufReader::new(file);
    let module_info: Vec<ModuleInfo> = serde_json::from_reader(reader).ok()?;
    Some(module_info)
}

pub fn write_modules(modules: &Vec<Module>) -> Result<(), StorageError> {
    let file = File::create(MODULE_CODES_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, modules)?;
    Ok(())
}

pub fn write_module_infos(module_infos: &Vec<ModuleInfo>) -> Result<(), StorageError> {
    let file = File::create(MODULE_INFO_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, module_infos)?;
    Ok(())
}
