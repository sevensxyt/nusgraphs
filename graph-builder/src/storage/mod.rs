use crate::errors::StorageError;
use crate::models::{Graph, Module, ModuleInfo};
use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub trait Storable: Serialize + DeserializeOwned {
    fn path() -> &'static str;
}

impl Storable for Module {
    fn path() -> &'static str {
        "data/modules.json"
    }
}

impl Storable for ModuleInfo {
    fn path() -> &'static str {
        "data/module_infos.json"
    }
}

impl Storable for Graph {
    fn path() -> &'static str {
        "data/graph.json"
    }
}

pub fn read_collection<T: Storable>() -> Option<Vec<T>> {
    let file = File::open(T::path()).ok()?;
    let reader = BufReader::new(file);
    let data: Vec<T> = serde_json::from_reader(reader).ok()?;
    Some(data)
}

pub fn write_collection<T: Storable>(data: &Vec<T>) -> Result<(), StorageError> {
    let file = File::create(T::path())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}

pub fn read_object<T: Storable>() -> Option<T> {
    let file = File::open(T::path()).ok()?;
    let reader = BufReader::new(file);
    let data: T = serde_json::from_reader(reader).ok()?;
    Some(data)
}

pub fn write_object<T: Storable>(data: &T) -> Result<(), StorageError> {
    let file = File::create(T::path())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}
