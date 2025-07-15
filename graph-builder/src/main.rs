mod api;
mod config;
mod errors;
mod graph;
mod models;
mod module;
mod storage;

use crate::config::Config;
use crate::errors::GraphBuilderError;

#[tokio::main]
async fn main() -> Result<(), GraphBuilderError> {
    let config = Config::new()?;

    if !graph::GraphTransformer::exists() {
        let module_infos = module::get_all_module_infos().await?;
        graph::GraphTransformer::new(module_infos, config).build()?;
        println!("Graph built");
    } else {
        println!("Graph found, skipping build");
    }

    Ok(())
}
