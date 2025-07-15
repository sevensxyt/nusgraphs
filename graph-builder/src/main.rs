mod api;
mod config;
mod errors;
mod graph;
mod models;
mod module;
mod storage;

use crate::config::Config;
use crate::errors::GraphBuilderError;
use crate::models::Graph;
use crate::storage::Storable;

#[tokio::main]
async fn main() -> Result<(), GraphBuilderError> {
    if graph::GraphTransformer::exists() {
        println!("Graph exists at {}", Graph::path());
    } else {
        let module_infos = module::get_all_module_infos().await?;
        let config = Config::new()?;
        graph::GraphTransformer::new(module_infos, config).build()?;
        println!("Graph building complete");
    }

    Ok(())
}
