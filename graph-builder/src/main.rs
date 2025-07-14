mod api;
mod errors;
mod models;
mod module;
mod storage;

use crate::errors::GraphBuilderError;

#[tokio::main]
async fn main() -> Result<(), GraphBuilderError> {
    let module_infos = module::get_all_module_infos().await?;
    println!("Done, loaded {} module infos", module_infos.len());
    Ok(())
}
